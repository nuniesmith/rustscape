//! Network handling - WebSocket connections and packet processing

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use tracing::{debug, error, info, warn};

use crate::game::{load_player, save_player, GameState, Player, Position};
use crate::world;

/// Handle a single WebSocket connection
pub async fn handle_connection(
    socket: WebSocket,
    state: Arc<GameState>,
    broadcast: broadcast::Sender<String>,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = broadcast.subscribe();

    // Create a channel for sending messages to this specific connection
    let (tx, mut rx) = mpsc::unbounded_channel::<ServerPacket>();

    // Connection state
    let mut player_id: Option<u32> = None;
    let mut username: Option<String> = None;

    info!("New WebSocket connection");

    // Send welcome message
    let welcome = ServerPacket::Welcome {
        message: "Welcome to Rustscape!".into(),
        tick: state.current_tick(),
    };
    if sender
        .send(Message::Text(
            serde_json::to_string(&welcome).unwrap().into(),
        ))
        .await
        .is_err()
    {
        return;
    }

    loop {
        tokio::select! {
            // Handle incoming messages from client
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<ClientPacket>(&text) {
                            Ok(packet) => {
                                let response = handle_packet(
                                    packet,
                                    &state,
                                    &mut player_id,
                                    &mut username,
                                    &broadcast,
                                    &tx,
                                ).await;

                                if let Some(response) = response {
                                    let json = serde_json::to_string(&response).unwrap();
                                    if sender.send(Message::Text(json.into())).await.is_err() {
                                        break;
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("Invalid packet: {}", e);
                            }
                        }
                    }
                    Some(Ok(Message::Binary(data))) => {
                        // Handle binary protocol if you want better performance
                        // For now, just use JSON
                        debug!("Received binary: {} bytes", data.len());
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        info!("Connection closed");
                        break;
                    }
                    Some(Ok(_)) => {} // Ping/Pong handled automatically
                    Some(Err(e)) => {
                        error!("WebSocket error: {}", e);
                        break;
                    }
                }
            }

            // Handle broadcast messages (chat, etc)
            msg = broadcast_rx.recv() => {
                if let Ok(msg) = msg {
                    if sender.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
            }

            // Handle direct messages to this player
            msg = rx.recv() => {
                if let Some(packet) = msg {
                    let json = serde_json::to_string(&packet).unwrap();
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    }

    // Cleanup on disconnect
    if let (Some(pid), Some(name)) = (player_id, username) {
        info!("Player {} disconnected", name);
        if let Some((_, player)) = state.players.remove(&pid) {
            let position = player.position;

            if let Err(e) = save_player(&player) {
                error!("Failed to save player on disconnect: {}", e);
            }

            // Notify nearby players that this player left
            send_to_visible_players(&state, &position, ServerPacket::PlayerLeft { id: pid });
        }
    }
}

async fn handle_packet(
    packet: ClientPacket,
    state: &GameState,
    player_id: &mut Option<u32>,
    username: &mut Option<String>,
    broadcast: &broadcast::Sender<String>,
    tx: &mpsc::UnboundedSender<ServerPacket>,
) -> Option<ServerPacket> {
    match packet {
        ClientPacket::Login {
            username: name,
            password: _,
        } => {
            // Simple auth - in production, hash passwords!
            info!("Login attempt: {}", name);

            // Try to load existing player or create new
            let mut player = load_player(&name).unwrap_or_else(|| {
                info!("Creating new player: {}", name);
                Player::new(state.next_id(), name.clone())
            });

            let pid = player.id;
            let pos = player.position;
            let skills = player.skills.clone();

            // Store the sender channel in the player for direct messaging
            player.sender = Some(tx.clone());

            // Store in game state
            state.players.insert(pid, player);

            *player_id = Some(pid);
            *username = Some(name.clone());

            // Notify nearby players that this player joined
            send_to_visible_players(
                state,
                &pos,
                ServerPacket::PlayerEnter {
                    id: pid,
                    username: name.clone(),
                    position: pos,
                },
            );

            // Send the new player a list of nearby players they can see
            let visible_players = world::get_visible_players(&pos, &state.players);
            let players: Vec<_> = visible_players
                .iter()
                .filter(|p| p.id != pid) // Exclude self
                .map(|p| PlayerInfo {
                    id: p.id,
                    username: p.username.clone(),
                    position: p.position,
                })
                .collect();

            if !players.is_empty() {
                let _ = tx.send(ServerPacket::PlayerList { players });
            }

            // Send nearby NPCs
            let visible_npcs = world::get_visible_npcs(&pos, &state.npcs);
            let npcs: Vec<_> = visible_npcs
                .iter()
                .map(|n| NpcInfo {
                    id: n.id,
                    def_id: n.def_id,
                    name: n.name.clone(),
                    position: n.position,
                })
                .collect();

            if !npcs.is_empty() {
                let _ = tx.send(ServerPacket::NpcList { npcs });
            }

            Some(ServerPacket::LoginSuccess {
                player_id: pid,
                username: name.clone(),
                position: pos,
                skills: SkillsData::from(&skills),
            })
        }

        ClientPacket::Move { x, y } => {
            if let Some(pid) = *player_id {
                if let Some(mut player) = state.players.get_mut(&pid) {
                    let old_position = player.position;
                    let new_position = Position {
                        x,
                        y,
                        z: old_position.z,
                    };

                    // Check collision before allowing movement
                    if world::can_move_to(&old_position, &new_position) {
                        player.position = new_position;
                        drop(player); // Release the lock before sending

                        // Get players who could see the old position
                        let old_viewers = world::get_visible_players(&old_position, &state.players);
                        let old_viewer_ids: std::collections::HashSet<u32> =
                            old_viewers.iter().map(|p| p.id).collect();

                        // Get players who can see the new position
                        let new_viewers = world::get_visible_players(&new_position, &state.players);
                        let new_viewer_ids: std::collections::HashSet<u32> =
                            new_viewers.iter().map(|p| p.id).collect();

                        // Send PlayerMoved to players who could see old OR new position
                        send_to_visible_players(
                            state,
                            &new_position,
                            ServerPacket::PlayerMoved {
                                player_id: pid,
                                new_position,
                            },
                        );

                        // Also send to players who saw the old position but not new
                        for viewer_id in old_viewer_ids.iter() {
                            if !new_viewer_ids.contains(viewer_id) && *viewer_id != pid {
                                if let Some(viewer) = state.players.get(viewer_id) {
                                    if let Some(sender) = &viewer.sender {
                                        let _ = sender.send(ServerPacket::PlayerMoved {
                                            player_id: pid,
                                            new_position,
                                        });
                                    }
                                }
                            }
                        }

                        // Send PlayerEnter to players who can see new position but couldn't see old
                        for viewer in new_viewers.iter() {
                            if !old_viewer_ids.contains(&viewer.id) && viewer.id != pid {
                                if let Some(sender) = &viewer.sender {
                                    if let Some(moving_player) = state.players.get(&pid) {
                                        let _ = sender.send(ServerPacket::PlayerEnter {
                                            id: pid,
                                            username: moving_player.username.clone(),
                                            position: new_position,
                                        });
                                    }
                                }
                            }
                        }

                        // Send PlayerLeave to players who could see old position but can't see new
                        for viewer_id in old_viewer_ids.iter() {
                            if !new_viewer_ids.contains(viewer_id) && *viewer_id != pid {
                                if let Some(viewer) = state.players.get(viewer_id) {
                                    if let Some(sender) = &viewer.sender {
                                        let _ = sender.send(ServerPacket::PlayerLeft { id: pid });
                                    }
                                }
                            }
                        }

                        debug!("Player {} moved to ({}, {})", pid, x, y);
                    } else {
                        // Movement blocked - send current position back to client
                        debug!(
                            "Player {} movement blocked: ({}, {}) -> ({}, {})",
                            pid, old_position.x, old_position.y, x, y
                        );

                        return Some(ServerPacket::PlayerMoved {
                            player_id: pid,
                            new_position: old_position,
                        });
                    }
                }
            }
            None
        }

        ClientPacket::Chat { message } => {
            if let Some(name) = username {
                info!("Chat from {}: {}", name, message);

                // Broadcast to all players
                let _ = broadcast.send(
                    serde_json::to_string(&ServerPacket::ChatMessage {
                        username: name.clone(),
                        message,
                    })
                    .unwrap(),
                );
            }
            None
        }

        ClientPacket::RequestPlayers => {
            // Send list of nearby players (using region-based visibility)
            if let Some(pid) = *player_id {
                if let Some(viewer) = state.players.get(&pid) {
                    let viewer_pos = viewer.position;

                    // Get only visible players using region system
                    let visible_players = world::get_visible_players(&viewer_pos, &state.players);

                    let players: Vec<_> = visible_players
                        .iter()
                        .filter(|p| p.id != pid) // Exclude self
                        .map(|p| PlayerInfo {
                            id: p.id,
                            username: p.username.clone(),
                            position: p.position,
                        })
                        .collect();

                    debug!("Player {} sees {} nearby players", pid, players.len());
                    return Some(ServerPacket::PlayerList { players });
                }
            }

            // Fallback if no player ID
            Some(ServerPacket::PlayerList { players: vec![] })
        }

        ClientPacket::RequestNpcs => {
            // Send nearby NPCs (using region-based visibility)
            if let Some(pid) = *player_id {
                if let Some(viewer) = state.players.get(&pid) {
                    let viewer_pos = viewer.position;

                    // Get only visible NPCs using region system
                    let visible_npcs = world::get_visible_npcs(&viewer_pos, &state.npcs);

                    let npcs: Vec<_> = visible_npcs
                        .iter()
                        .map(|n| NpcInfo {
                            id: n.id,
                            def_id: n.def_id,
                            name: n.name.clone(),
                            position: n.position,
                        })
                        .collect();

                    debug!("Player {} sees {} nearby NPCs", pid, npcs.len());
                    return Some(ServerPacket::NpcList { npcs });
                }
            }

            // Fallback if no player ID
            Some(ServerPacket::NpcList { npcs: vec![] })
        }

        ClientPacket::RequestGroundItems => {
            // Send nearby ground items (using visibility)
            if let Some(pid) = *player_id {
                if let Some(viewer) = state.players.get(&pid) {
                    let viewer_pos = viewer.position;

                    // Get only visible ground items (within 15 tiles)
                    let visible_items: Vec<_> = state
                        .ground_items
                        .iter()
                        .filter(|item| world::in_view_distance(&viewer_pos, &item.position))
                        .map(|item| GroundItemInfo {
                            id: item.id,
                            item_id: item.item_id,
                            amount: item.amount,
                            position: item.position,
                        })
                        .collect();

                    debug!("Player {} sees {} ground items", pid, visible_items.len());
                    return Some(ServerPacket::GroundItemList {
                        items: visible_items,
                    });
                }
            }

            // Fallback if no player ID
            Some(ServerPacket::GroundItemList { items: vec![] })
        }

        ClientPacket::DropItem { slot } => {
            if let Some(pid) = *player_id {
                if let Some(mut player) = state.players.get_mut(&pid) {
                    // Check if slot is valid and has an item
                    if slot < player.inventory.len() {
                        if let Some(item) = player.inventory[slot].take() {
                            let position = player.position;
                            drop(player); // Release lock before creating ground item

                            // Create ground item
                            let ground_item = crate::game::GroundItem {
                                id: state.next_id(),
                                item_id: item.id,
                                amount: item.amount,
                                position,
                                owner_id: Some(pid),
                                spawn_tick: state.current_tick(),
                            };

                            let ground_item_info = GroundItemInfo {
                                id: ground_item.id,
                                item_id: ground_item.item_id,
                                amount: ground_item.amount,
                                position: ground_item.position,
                            };

                            state.ground_items.insert(ground_item.id, ground_item);

                            info!(
                                "Player {} dropped item {} at ({}, {})",
                                pid, item.id, position.x, position.y
                            );

                            // Notify nearby players
                            send_to_visible_players(
                                state,
                                &position,
                                ServerPacket::GroundItemSpawned {
                                    item: ground_item_info.clone(),
                                },
                            );

                            return Some(ServerPacket::GroundItemSpawned {
                                item: ground_item_info,
                            });
                        }
                    }
                }
            }
            None
        }

        ClientPacket::PickupItem { ground_item_id } => {
            if let Some(pid) = *player_id {
                if let Some(ground_item) = state.ground_items.get(&ground_item_id) {
                    let position = ground_item.position;
                    let item_id = ground_item.item_id;
                    let amount = ground_item.amount;

                    // Check if player is close enough (within 1 tile)
                    if let Some(player) = state.players.get(&pid) {
                        let player_pos = player.position;
                        let dx = (player_pos.x - position.x).abs();
                        let dy = (player_pos.y - position.y).abs();

                        if dx <= 1 && dy <= 1 && player_pos.z == position.z {
                            // Close enough to pick up
                            drop(player);
                            drop(ground_item);

                            // Remove ground item
                            if let Some((_, removed_item)) =
                                state.ground_items.remove(&ground_item_id)
                            {
                                // Add to player inventory
                                if let Some(mut player) = state.players.get_mut(&pid) {
                                    // Try to stack with existing item
                                    let mut added = false;
                                    for inv_slot in player.inventory.iter_mut() {
                                        if let Some(existing) = inv_slot {
                                            if existing.id == item_id {
                                                existing.amount += amount;
                                                added = true;
                                                break;
                                            }
                                        }
                                    }

                                    // If not stacked, find empty slot
                                    if !added {
                                        for inv_slot in player.inventory.iter_mut() {
                                            if inv_slot.is_none() {
                                                *inv_slot = Some(crate::game::Item {
                                                    id: item_id,
                                                    amount,
                                                });
                                                added = true;
                                                break;
                                            }
                                        }
                                    }

                                    if added {
                                        info!(
                                            "Player {} picked up item {} (amount: {})",
                                            pid, item_id, amount
                                        );

                                        // Notify nearby players
                                        send_to_visible_players(
                                            state,
                                            &position,
                                            ServerPacket::GroundItemRemoved { ground_item_id },
                                        );

                                        return Some(ServerPacket::GroundItemRemoved {
                                            ground_item_id,
                                        });
                                    } else {
                                        // Inventory full - put item back
                                        state.ground_items.insert(ground_item_id, removed_item);
                                        warn!("Player {} inventory full, can't pick up item", pid);
                                    }
                                }
                            }
                        } else {
                            debug!("Player {} too far from item to pick up", pid);
                        }
                    }
                }
            }
            None
        }

        ClientPacket::TalkToNpc { npc_id } => {
            if let Some(pid) = *player_id {
                // Find the NPC
                if let Some(npc) = state.npcs.get(&npc_id) {
                    let npc_pos = npc.position;
                    drop(npc);

                    // Check if player is close enough (within 3 tiles)
                    if let Some(player) = state.players.get(&pid) {
                        let player_pos = player.position;
                        let dx = (player_pos.x - npc_pos.x).abs();
                        let dy = (player_pos.y - npc_pos.y).abs();

                        if dx <= 3 && dy <= 3 && player_pos.z == npc_pos.z {
                            // Close enough to talk
                            drop(player);

                            // Get dialogue for this NPC
                            if let Some(npc_dialogue) = state.dialogues.get(&npc_id) {
                                // Get the greeting dialogue (id: 0)
                                if let Some(dialogue) =
                                    npc_dialogue.dialogues.iter().find(|d| d.id == 0)
                                {
                                    let options: Vec<DialogueOptionInfo> = dialogue
                                        .options
                                        .iter()
                                        .enumerate()
                                        .map(|(idx, opt)| DialogueOptionInfo {
                                            text: opt.text.clone(),
                                            option_index: idx,
                                        })
                                        .collect();

                                    info!("Player {} talked to NPC {}", pid, npc_dialogue.npc_name);

                                    return Some(ServerPacket::NpcDialogue {
                                        npc_id,
                                        npc_name: npc_dialogue.npc_name.clone(),
                                        dialogue_id: dialogue.id,
                                        text: dialogue.text.clone(),
                                        options,
                                    });
                                }
                            } else {
                                warn!("No dialogue found for NPC ID {}", npc_id);
                            }
                        } else {
                            debug!("Player {} too far from NPC to talk", pid);
                        }
                    }
                }
            }
            None
        }

        ClientPacket::SelectDialogueOption {
            npc_id,
            dialogue_id,
            option_index,
        } => {
            if let Some(pid) = *player_id {
                // Get the current dialogue to find next dialogue ID
                if let Some(npc_dialogue) = state.dialogues.get(&npc_id) {
                    if let Some(current_dialogue) =
                        npc_dialogue.dialogues.iter().find(|d| d.id == dialogue_id)
                    {
                        if option_index < current_dialogue.options.len() {
                            let selected_option = &current_dialogue.options[option_index];

                            // Check if there's a next dialogue
                            if let Some(next_id) = selected_option.next_dialogue {
                                // Find the next dialogue
                                if let Some(next_dialogue) =
                                    npc_dialogue.dialogues.iter().find(|d| d.id == next_id)
                                {
                                    let options: Vec<DialogueOptionInfo> = next_dialogue
                                        .options
                                        .iter()
                                        .enumerate()
                                        .map(|(idx, opt)| DialogueOptionInfo {
                                            text: opt.text.clone(),
                                            option_index: idx,
                                        })
                                        .collect();

                                    return Some(ServerPacket::NpcDialogue {
                                        npc_id,
                                        npc_name: npc_dialogue.npc_name.clone(),
                                        dialogue_id: next_dialogue.id,
                                        text: next_dialogue.text.clone(),
                                        options,
                                    });
                                }
                            } else {
                                // End of dialogue
                                debug!("Player {} ended dialogue with NPC {}", pid, npc_id);
                            }
                        }
                    }
                }
            }
            None
        }

        ClientPacket::ExamineItem { item_id } => {
            // Look up the item definition
            if let Some(item_def) = state.items.iter().find(|i| i.id == item_id) {
                info!(
                    "Player {:?} examined item: {} ({})",
                    player_id, item_def.name, item_id
                );
                Some(ServerPacket::ExamineText {
                    text: item_def.examine.clone(),
                })
            } else {
                warn!(
                    "Player {:?} tried to examine unknown item ID: {}",
                    player_id, item_id
                );
                Some(ServerPacket::ExamineText {
                    text: "Nothing interesting happens.".to_string(),
                })
            }
        }

        ClientPacket::ExamineNpc { npc_id } => {
            // Look up the NPC definition from the NPC instance
            if let Some(npc) = state.npcs.get(&npc_id) {
                let def_id = npc.def_id;
                drop(npc);

                // Find the NPC definition
                if let Some(npc_def) = state.npc_defs.iter().find(|n| n.id == def_id) {
                    info!(
                        "Player {:?} examined NPC: {} ({})",
                        player_id, npc_def.name, npc_id
                    );
                    Some(ServerPacket::ExamineText {
                        text: npc_def.examine.clone(),
                    })
                } else {
                    warn!("NPC {} has invalid def_id: {}", npc_id, def_id);
                    Some(ServerPacket::ExamineText {
                        text: "Nothing interesting happens.".to_string(),
                    })
                }
            } else {
                warn!(
                    "Player {:?} tried to examine unknown NPC ID: {}",
                    player_id, npc_id
                );
                Some(ServerPacket::ExamineText {
                    text: "Nothing interesting happens.".to_string(),
                })
            }
        }

        ClientPacket::Attack {
            target_type,
            target_id,
        } => {
            if let Some(pid) = *player_id {
                if let Some(mut attacker) = state.players.get_mut(&pid) {
                    let attacker_pos = attacker.position;
                    let attacker_attack = attacker.skills.attack.level;
                    let attacker_strength = attacker.skills.strength.level;
                    let attacker_name = attacker.username.clone();

                    // Check if already in combat (cooldown of 4 ticks = 2.4 seconds)
                    let current_tick = state.current_tick();
                    if current_tick - attacker.last_combat_tick < 4 {
                        debug!("Player {} attack on cooldown", pid);
                        return None;
                    }

                    drop(attacker); // Release lock before processing

                    match target_type.as_str() {
                        "player" => {
                            if let Some(mut target) = state.players.get_mut(&target_id) {
                                let target_pos = target.position;
                                let target_defence = target.skills.defence.level;
                                let target_hp = target.current_hp;
                                let target_max_hp =
                                    crate::game::get_max_hp(target.skills.hitpoints.level);
                                let target_name = target.username.clone();

                                // Check range (within 1 tile)
                                if !crate::game::in_combat_range(&attacker_pos, &target_pos) {
                                    debug!("Player {} too far to attack player {}", pid, target_id);
                                    return None;
                                }

                                // Calculate hit and damage
                                let hit =
                                    crate::game::calculate_hit(attacker_attack, target_defence);
                                let damage =
                                    crate::game::calculate_damage(attacker_strength, 0, hit);

                                // Apply damage
                                let new_hp = target_hp.saturating_sub(damage);
                                target.current_hp = new_hp;
                                drop(target);

                                // Update attacker's last combat tick and award XP
                                if let Some(mut attacker) = state.players.get_mut(&pid) {
                                    attacker.last_combat_tick = current_tick;
                                    attacker.combat_target =
                                        Some(crate::game::CombatTarget::Player(target_id));

                                    // Award combat XP (only if damage > 0)
                                    if damage > 0 {
                                        let xp = crate::game::calculate_combat_xp(damage);

                                        // Award XP to Attack, Strength, Defence, and Hitpoints
                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.attack, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Attack".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.strength, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Strength".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.defence, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Defence".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.hitpoints, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Hitpoints".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        // Send XP gain notification
                                        if let Some(sender) = &attacker.sender {
                                            let _ = sender.send(ServerPacket::XpGain {
                                                skill_name: "Combat".to_string(),
                                                xp_gained: xp,
                                                total_xp: attacker.skills.attack.xp,
                                            });
                                        }
                                    }
                                }

                                info!(
                                    "Combat: {} attacked {} for {} damage (hit: {})",
                                    attacker_name, target_name, damage, hit
                                );

                                // Broadcast combat hit to nearby players
                                send_to_visible_players(
                                    state,
                                    &attacker_pos,
                                    ServerPacket::CombatHit {
                                        attacker_id: pid,
                                        target_id,
                                        damage,
                                        target_hp: new_hp,
                                        target_max_hp,
                                    },
                                );

                                // Check for death
                                if new_hp == 0 {
                                    info!(
                                        "Player {} died to player {}",
                                        target_name, attacker_name
                                    );

                                    // Reset target to spawn
                                    if let Some(mut target) = state.players.get_mut(&target_id) {
                                        target.position = Position {
                                            x: 3222,
                                            y: 3218,
                                            z: 0,
                                        }; // Lumbridge spawn
                                        target.current_hp = target_max_hp;
                                        target.combat_target = None;
                                    }

                                    // Broadcast death
                                    send_to_visible_players(
                                        state,
                                        &target_pos,
                                        ServerPacket::Death {
                                            entity_id: target_id,
                                            killer_id: Some(pid),
                                        },
                                    );
                                }

                                return Some(ServerPacket::CombatHit {
                                    attacker_id: pid,
                                    target_id,
                                    damage,
                                    target_hp: new_hp,
                                    target_max_hp,
                                });
                            }
                        }
                        "npc" => {
                            if let Some(mut target_npc) = state.npcs.get_mut(&target_id) {
                                let target_pos = target_npc.position;
                                let target_hp = target_npc.health;
                                let target_max_hp = target_npc.max_health;
                                let target_name = target_npc.name.clone();
                                let target_npc_def_id = target_npc.def_id;

                                // Check range
                                if !crate::game::in_combat_range(&attacker_pos, &target_pos) {
                                    debug!("Player {} too far to attack NPC {}", pid, target_id);
                                    return None;
                                }

                                // NPCs have base 10 defence for now
                                let hit = crate::game::calculate_hit(attacker_attack, 10);
                                let damage =
                                    crate::game::calculate_damage(attacker_strength, 0, hit);

                                // Apply damage
                                let new_hp = target_hp.saturating_sub(damage);
                                target_npc.health = new_hp;
                                drop(target_npc);

                                // Set NPC to retaliate (auto-attack back)
                                if let Some(mut target_npc) = state.npcs.get_mut(&target_id) {
                                    target_npc.combat_target = Some(pid);
                                }

                                // Update attacker's last combat tick and award XP
                                if let Some(mut attacker) = state.players.get_mut(&pid) {
                                    attacker.last_combat_tick = current_tick;
                                    attacker.combat_target =
                                        Some(crate::game::CombatTarget::Npc(target_id));

                                    // Award combat XP (only if damage > 0)
                                    if damage > 0 {
                                        let xp = crate::game::calculate_combat_xp(damage);

                                        // Award XP to Attack, Strength, Defence, and Hitpoints
                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.attack, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Attack".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.strength, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Strength".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.defence, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Defence".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        if let Some(new_lvl) =
                                            crate::game::add_xp(&mut attacker.skills.hitpoints, xp)
                                        {
                                            if let Some(sender) = &attacker.sender {
                                                let _ = sender.send(ServerPacket::LevelUp {
                                                    skill_name: "Hitpoints".to_string(),
                                                    new_level: new_lvl,
                                                });
                                            }
                                        }

                                        // Send XP gain notification
                                        if let Some(sender) = &attacker.sender {
                                            let _ = sender.send(ServerPacket::XpGain {
                                                skill_name: "Combat".to_string(),
                                                xp_gained: xp,
                                                total_xp: attacker.skills.attack.xp,
                                            });
                                        }
                                    }
                                }

                                info!(
                                    "Combat: {} attacked NPC {} for {} damage",
                                    attacker_name, target_name, damage
                                );

                                // Broadcast combat hit
                                send_to_visible_players(
                                    state,
                                    &attacker_pos,
                                    ServerPacket::CombatHit {
                                        attacker_id: pid,
                                        target_id,
                                        damage,
                                        target_hp: new_hp,
                                        target_max_hp,
                                    },
                                );

                                // Check for NPC death
                                if new_hp == 0 {
                                    info!("NPC {} killed by {}", target_name, attacker_name);

                                    // Generate loot drops
                                    if let Some(npc_def) =
                                        state.npc_defs.iter().find(|d| d.id == target_npc_def_id)
                                    {
                                        let loot = crate::game::generate_loot(&npc_def.loot_table);

                                        // Spawn loot as ground items
                                        for (item_id, amount) in loot {
                                            let ground_item = crate::game::GroundItem {
                                                id: state.next_id(),
                                                item_id,
                                                amount,
                                                position: target_pos,
                                                owner_id: Some(pid),
                                                spawn_tick: state.current_tick(),
                                            };

                                            let ground_item_id = ground_item.id;
                                            state
                                                .ground_items
                                                .insert(ground_item_id, ground_item.clone());

                                            // Notify nearby players
                                            send_to_visible_players(
                                                state,
                                                &target_pos,
                                                ServerPacket::GroundItemSpawned {
                                                    item: crate::net::GroundItemInfo {
                                                        id: ground_item.id,
                                                        item_id: ground_item.item_id,
                                                        amount: ground_item.amount,
                                                        position: ground_item.position,
                                                    },
                                                },
                                            );
                                        }
                                    }

                                    // Schedule NPC respawn (10 ticks = 6 seconds)
                                    if let Some(mut npc) = state.npcs.get_mut(&target_id) {
                                        npc.respawn_tick = Some(state.current_tick() + 10);
                                        npc.combat_target = None;
                                    }

                                    // Broadcast death
                                    send_to_visible_players(
                                        state,
                                        &target_pos,
                                        ServerPacket::Death {
                                            entity_id: target_id,
                                            killer_id: Some(pid),
                                        },
                                    );
                                }

                                return Some(ServerPacket::CombatHit {
                                    attacker_id: pid,
                                    target_id,
                                    damage,
                                    target_hp: new_hp,
                                    target_max_hp,
                                });
                            }
                        }
                        _ => {
                            warn!("Invalid target type: {}", target_type);
                        }
                    }
                }
            }
            None
        }

        ClientPacket::Ping { timestamp } => Some(ServerPacket::Pong {
            timestamp,
            server_tick: state.current_tick(),
        }),
    }
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Send a packet to all players who can see a given position
fn send_to_visible_players(state: &GameState, position: &Position, packet: ServerPacket) {
    let visible_players = world::get_visible_players(position, &state.players);

    for player in visible_players.iter() {
        if let Some(sender) = &player.sender {
            let _ = sender.send(packet.clone());
        }
    }
}

// ============================================================================
// Packet Definitions (JSON-based for simplicity)
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPacket {
    Login {
        username: String,
        password: String,
    },
    Move {
        x: i32,
        y: i32,
    },
    Chat {
        message: String,
    },
    RequestPlayers,
    RequestNpcs,
    RequestGroundItems,
    DropItem {
        slot: usize,
    },
    PickupItem {
        ground_item_id: u32,
    },
    TalkToNpc {
        npc_id: u32,
    },
    SelectDialogueOption {
        npc_id: u32,
        dialogue_id: u32,
        option_index: usize,
    },
    ExamineItem {
        item_id: u32,
    },
    ExamineNpc {
        npc_id: u32,
    },
    Attack {
        target_type: String, // "player" or "npc"
        target_id: u32,
    },
    Ping {
        timestamp: u64,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ServerPacket {
    Welcome {
        message: String,
        tick: u32,
    },
    LoginSuccess {
        player_id: u32,
        username: String,
        position: Position,
        skills: SkillsData,
    },
    LoginFailed {
        reason: String,
    },
    PlayerEnter {
        id: u32,
        username: String,
        position: Position,
    },
    PlayerLeft {
        id: u32,
    },
    PlayerMoved {
        player_id: u32,
        new_position: Position,
    },
    PlayerList {
        players: Vec<PlayerInfo>,
    },
    NpcList {
        npcs: Vec<NpcInfo>,
    },
    ChatMessage {
        username: String,
        message: String,
    },
    Pong {
        timestamp: u64,
        server_tick: u32,
    },
    GroundItemSpawned {
        item: GroundItemInfo,
    },
    GroundItemRemoved {
        ground_item_id: u32,
    },
    GroundItemList {
        items: Vec<GroundItemInfo>,
    },
    NpcDialogue {
        npc_id: u32,
        npc_name: String,
        dialogue_id: u32,
        text: String,
        options: Vec<DialogueOptionInfo>,
    },
    ExamineText {
        text: String,
    },
    CombatHit {
        attacker_id: u32,
        target_id: u32,
        damage: u32,
        target_hp: u32,
        target_max_hp: u32,
    },
    Death {
        entity_id: u32,
        killer_id: Option<u32>,
    },
    HealthUpdate {
        entity_id: u32,
        current_hp: u32,
        max_hp: u32,
    },
    XpGain {
        skill_name: String,
        xp_gained: u32,
        total_xp: u32,
    },
    LevelUp {
        skill_name: String,
        new_level: u8,
    },
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerInfo {
    pub id: u32,
    pub username: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize)]
pub struct DialogueOptionInfo {
    pub text: String,
    pub option_index: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct NpcInfo {
    pub id: u32,
    pub def_id: u32,
    pub name: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize)]
pub struct GroundItemInfo {
    pub id: u32,
    pub item_id: u32,
    pub amount: u32,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize)]
pub struct SkillsData {
    pub attack: (u8, u32),
    pub defence: (u8, u32),
    pub strength: (u8, u32),
    pub hitpoints: (u8, u32),
    pub ranged: (u8, u32),
    pub prayer: (u8, u32),
    pub magic: (u8, u32),
    pub cooking: (u8, u32),
    pub woodcutting: (u8, u32),
    pub fletching: (u8, u32),
    pub fishing: (u8, u32),
    pub firemaking: (u8, u32),
    pub crafting: (u8, u32),
    pub smithing: (u8, u32),
    pub mining: (u8, u32),
    pub herblore: (u8, u32),
    pub agility: (u8, u32),
    pub thieving: (u8, u32),
    pub slayer: (u8, u32),
    pub farming: (u8, u32),
    pub runecrafting: (u8, u32),
    pub hunter: (u8, u32),
    pub construction: (u8, u32),
}

impl From<&crate::game::Skills> for SkillsData {
    fn from(s: &crate::game::Skills) -> Self {
        Self {
            attack: (s.attack.level, s.attack.xp),
            defence: (s.defence.level, s.defence.xp),
            strength: (s.strength.level, s.strength.xp),
            hitpoints: (s.hitpoints.level, s.hitpoints.xp),
            ranged: (s.ranged.level, s.ranged.xp),
            prayer: (s.prayer.level, s.prayer.xp),
            magic: (s.magic.level, s.magic.xp),
            cooking: (s.cooking.level, s.cooking.xp),
            woodcutting: (s.woodcutting.level, s.woodcutting.xp),
            fletching: (s.fletching.level, s.fletching.xp),
            fishing: (s.fishing.level, s.fishing.xp),
            firemaking: (s.firemaking.level, s.firemaking.xp),
            crafting: (s.crafting.level, s.crafting.xp),
            smithing: (s.smithing.level, s.smithing.xp),
            mining: (s.mining.level, s.mining.xp),
            herblore: (s.herblore.level, s.herblore.xp),
            agility: (s.agility.level, s.agility.xp),
            thieving: (s.thieving.level, s.thieving.xp),
            slayer: (s.slayer.level, s.slayer.xp),
            farming: (s.farming.level, s.farming.xp),
            runecrafting: (s.runecrafting.level, s.runecrafting.xp),
            hunter: (s.hunter.level, s.hunter.xp),
            construction: (s.construction.level, s.construction.xp),
        }
    }
}
