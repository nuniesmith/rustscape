//! Network handling - WebSocket connections and packet processing

use axum::extract::ws::{Message, WebSocket};
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;
use tracing::{debug, error, info, warn};

use crate::game::{load_player, save_player, GameState, Player, Position};

/// Handle a single WebSocket connection
pub async fn handle_connection(
    socket: WebSocket,
    state: Arc<GameState>,
    broadcast: broadcast::Sender<String>,
) {
    let (mut sender, mut receiver) = socket.split();
    let mut broadcast_rx = broadcast.subscribe();

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
        }
    }

    // Cleanup on disconnect
    if let (Some(pid), Some(name)) = (player_id, username) {
        info!("Player {} disconnected", name);
        if let Some((_, player)) = state.players.remove(&pid) {
            if let Err(e) = save_player(&player) {
                error!("Failed to save player on disconnect: {}", e);
            }
        }
    }
}

async fn handle_packet(
    packet: ClientPacket,
    state: &GameState,
    player_id: &mut Option<u32>,
    username: &mut Option<String>,
    broadcast: &broadcast::Sender<String>,
) -> Option<ServerPacket> {
    match packet {
        ClientPacket::Login {
            username: name,
            password: _,
        } => {
            // Simple auth - in production, hash passwords!
            info!("Login attempt: {}", name);

            // Try to load existing player or create new
            let player = load_player(&name).unwrap_or_else(|| {
                info!("Creating new player: {}", name);
                Player::new(state.next_id(), name.clone())
            });

            let pid = player.id;
            let pos = player.position;
            let skills = player.skills.clone();

            // Store in game state
            state.players.insert(pid, player);

            *player_id = Some(pid);
            *username = Some(name.clone());

            // Notify others
            let _ = broadcast.send(
                serde_json::to_string(&ServerPacket::PlayerJoined {
                    id: pid,
                    username: name.clone(),
                    position: pos,
                })
                .unwrap(),
            );

            Some(ServerPacket::LoginSuccess {
                player_id: pid,
                position: pos,
                skills: SkillsData::from(&skills),
            })
        }

        ClientPacket::Move { x, y } => {
            if let Some(pid) = *player_id {
                if let Some(mut player) = state.players.get_mut(&pid) {
                    // TODO: Validate movement, check collision
                    player.position.x = x;
                    player.position.y = y;

                    // Broadcast movement to others
                    let _ = broadcast.send(
                        serde_json::to_string(&ServerPacket::PlayerMoved {
                            id: pid,
                            position: player.position,
                        })
                        .unwrap(),
                    );
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
            // Send list of nearby players
            let players: Vec<_> = state
                .players
                .iter()
                .filter(|p| Some(p.id) != *player_id) // Exclude self
                .map(|p| PlayerInfo {
                    id: p.id,
                    username: p.username.clone(),
                    position: p.position,
                })
                .collect();

            Some(ServerPacket::PlayerList { players })
        }

        ClientPacket::RequestNpcs => {
            // Send nearby NPCs
            let npcs: Vec<_> = state
                .npcs
                .iter()
                .map(|n| NpcInfo {
                    id: n.id,
                    def_id: n.def_id,
                    name: n.name.clone(),
                    position: n.position,
                })
                .collect();

            Some(ServerPacket::NpcList { npcs })
        }

        ClientPacket::Ping { timestamp } => Some(ServerPacket::Pong {
            timestamp,
            server_tick: state.current_tick(),
        }),
    }
}

// ============================================================================
// Packet Definitions (JSON-based for simplicity)
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientPacket {
    Login { username: String, password: String },
    Move { x: i32, y: i32 },
    Chat { message: String },
    RequestPlayers,
    RequestNpcs,
    Ping { timestamp: u64 },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ServerPacket {
    Welcome {
        message: String,
        tick: u32,
    },
    LoginSuccess {
        player_id: u32,
        position: Position,
        skills: SkillsData,
    },
    LoginFailed {
        reason: String,
    },
    PlayerJoined {
        id: u32,
        username: String,
        position: Position,
    },
    PlayerLeft {
        id: u32,
    },
    PlayerMoved {
        id: u32,
        position: Position,
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
}

#[derive(Debug, Serialize)]
pub struct PlayerInfo {
    pub id: u32,
    pub username: String,
    pub position: Position,
}

#[derive(Debug, Serialize)]
pub struct NpcInfo {
    pub id: u32,
    pub def_id: u32,
    pub name: String,
    pub position: Position,
}

#[derive(Debug, Serialize)]
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
