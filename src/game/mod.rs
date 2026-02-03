//! Game state and tick loop

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};
use tokio::sync::mpsc;
use tracing::{debug, info, warn};

/// Global game state - shared across all connections
pub struct GameState {
    /// Online players (concurrent hashmap, no locks needed)
    pub players: DashMap<u32, Player>,

    /// Item definitions (loaded once at startup)
    pub items: Vec<ItemDef>,

    /// NPC definitions
    pub npc_defs: Vec<NpcDef>,

    /// Active NPCs in the world
    pub npcs: DashMap<u32, Npc>,

    /// Ground items (items on the ground)
    /// Key: unique ground item ID
    pub ground_items: DashMap<u32, GroundItem>,

    /// NPC dialogues (loaded from JSON)
    /// Key: NPC ID
    pub dialogues: DashMap<u32, NpcDialogue>,

    /// Next entity ID
    next_id: AtomicU32,

    /// Current game tick
    pub tick: AtomicU32,
}

impl GameState {
    /// Load all game data from JSON files
    pub fn load_from_files() -> Self {
        let items = load_json_or_default("assets/definitions/items.json");
        let npc_defs: Vec<NpcDef> = load_json_or_default("assets/definitions/npcs.json");
        let npc_spawns: Vec<NpcSpawn> = load_all_json_in_dir("assets/spawns/npcs");
        let dialogues_vec: Vec<NpcDialogue> = load_all_json_in_dir("assets/dialogue/npcs");

        let state = Self {
            players: DashMap::new(),
            items,
            npc_defs: npc_defs.clone(),
            npcs: DashMap::new(),
            ground_items: DashMap::new(),
            dialogues: DashMap::new(),
            next_id: AtomicU32::new(1),
            tick: AtomicU32::new(0),
        };

        // Load dialogues into DashMap
        for dialogue in dialogues_vec {
            info!("Loaded dialogue for NPC: {}", dialogue.npc_name);
            state.dialogues.insert(dialogue.npc_id, dialogue);
        }

        // Spawn NPCs from spawn files
        for spawn in npc_spawns {
            if let Some(def) = npc_defs.iter().find(|d| d.id == spawn.npc_id) {
                let max_hp = (def.hitpoints as u32) * 10;
                let npc = Npc {
                    id: state.next_id(),
                    def_id: spawn.npc_id,
                    name: def.name.clone(),
                    position: spawn.position,
                    spawn_position: spawn.position,
                    health: max_hp,
                    max_health: max_hp,
                    combat_target: None,
                    last_combat_tick: 0,
                    is_aggressive: def.is_aggressive,
                    aggro_range: def.aggro_range,
                    respawn_tick: None,
                };
                state.npcs.insert(npc.id, npc);
            } else {
                warn!("No NPC definition found for ID {}", spawn.npc_id);
            }
        }

        info!("Spawned {} NPCs", state.npcs.len());
        state
    }

    pub fn next_id(&self) -> u32 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn current_tick(&self) -> u32 {
        self.tick.load(Ordering::Relaxed)
    }
}

/// Main game tick loop - runs every 600ms
pub async fn tick_loop(state: std::sync::Arc<GameState>) {
    let mut interval = tokio::time::interval(Duration::from_millis(600));

    loop {
        interval.tick().await;
        let tick = state.tick.fetch_add(1, Ordering::Relaxed);

        // Process NPC AI (aggression, auto-retaliate, respawn)
        let npc_ids: Vec<u32> = state.npcs.iter().map(|entry| *entry.key()).collect();
        for npc_id in npc_ids {
            process_npc(&state, npc_id, tick);
        }

        // Handle NPC respawns
        let npc_def_map: std::collections::HashMap<u32, &NpcDef> =
            state.npc_defs.iter().map(|def| (def.id, def)).collect();

        // Check for NPCs that need to respawn
        let mut to_respawn = Vec::new();
        for npc in state.npcs.iter() {
            if let Some(respawn_tick) = npc.respawn_tick {
                if tick >= respawn_tick {
                    to_respawn.push((*npc.key(), npc.def_id, npc.spawn_position, npc.max_health));
                }
            }
        }

        // Respawn NPCs
        for (npc_id, def_id, spawn_pos, max_hp) in to_respawn {
            if let Some(mut npc) = state.npcs.get_mut(&npc_id) {
                npc.position = spawn_pos;
                npc.health = max_hp;
                npc.combat_target = None;
                npc.last_combat_tick = 0;
                npc.respawn_tick = None;

                // Broadcast NPC respawn (re-use NpcList for now)
                info!("NPC {} respawned at {:?}", npc.name, spawn_pos);
            }
        }

        // Process player actions (combat, skills, etc)
        for mut player in state.players.iter_mut() {
            process_player(&mut player);
        }

        // Every 100 ticks (60 seconds), save all players
        if tick % 100 == 0 {
            save_all_players(&state);
        }

        // Debug log every 10 ticks
        if tick % 10 == 0 {
            debug!(
                "Tick {}: {} players, {} npcs",
                tick,
                state.players.len(),
                state.npcs.len()
            );
        }
    }
}

fn process_npc(state: &GameState, npc_id: u32, current_tick: u32) {
    // Get NPC data
    let mut npc_data = if let Some(npc) = state.npcs.get(&npc_id) {
        (
            npc.position,
            npc.combat_target,
            npc.last_combat_tick,
            npc.is_aggressive,
            npc.aggro_range,
            npc.def_id,
        )
    } else {
        return;
    };

    let (npc_pos, mut target_opt, last_tick, is_aggressive, aggro_range, def_id) = npc_data;

    // Get NPC stats from definition
    let (npc_attack, npc_strength, _npc_defence) = state
        .npc_defs
        .iter()
        .find(|d| d.id == def_id)
        .map(|d| (d.attack, d.strength, d.defence))
        .unwrap_or((1, 1, 1));

    // Check if NPC should attack (has target or is aggressive)
    let should_attack = if let Some(target_id) = target_opt {
        // Has existing target - continue attacking
        if let Some(player) = state.players.get(&target_id) {
            in_combat_range(&npc_pos, &player.position) && current_tick - last_tick >= 4
        } else {
            false
        }
    } else if is_aggressive {
        // No target but aggressive - find nearby players
        for player in state.players.iter() {
            let distance = ((npc_pos.x - player.position.x).pow(2)
                + (npc_pos.y - player.position.y).pow(2)) as f32;
            let distance = distance.sqrt() as u32;

            if distance <= aggro_range && npc_pos.z == player.position.z {
                target_opt = Some(*player.key());
                break;
            }
        }
        false // Don't attack this tick, just acquired target
    } else {
        false
    };

    if should_attack {
        if let Some(target_id) = target_opt {
            npc_attack_player(
                state,
                npc_id,
                target_id,
                current_tick,
                npc_attack,
                npc_strength,
            );
        }
    }

    // Update NPC's target if it changed
    if let Some(mut npc) = state.npcs.get_mut(&npc_id) {
        npc.combat_target = target_opt;
    }
}

fn npc_attack_player(
    state: &GameState,
    npc_id: u32,
    player_id: u32,
    current_tick: u32,
    npc_attack: u8,
    npc_strength: u8,
) {
    use crate::net::ServerPacket;

    // Get player data
    let (player_pos, player_defence, player_hp, player_max_hp, player_sender) =
        if let Some(player) = state.players.get(&player_id) {
            (
                player.position,
                player.skills.defence.level,
                player.current_hp,
                get_max_hp(player.skills.hitpoints.level),
                player.sender.clone(),
            )
        } else {
            return;
        };

    // Get NPC position
    let npc_pos = if let Some(npc) = state.npcs.get(&npc_id) {
        npc.position
    } else {
        return;
    };

    // Check range
    if !in_combat_range(&npc_pos, &player_pos) {
        return;
    }

    // Calculate hit and damage
    let hit = calculate_hit(npc_attack, player_defence);
    let damage = calculate_damage(npc_strength, 0, hit);

    // Apply damage to player
    let new_hp = player_hp.saturating_sub(damage);
    if let Some(mut player) = state.players.get_mut(&player_id) {
        player.current_hp = new_hp;
    }

    // Update NPC's last combat tick
    if let Some(mut npc) = state.npcs.get_mut(&npc_id) {
        npc.last_combat_tick = current_tick;
    }

    // Broadcast combat hit
    send_to_visible_players_from_pos(
        state,
        &npc_pos,
        crate::net::ServerPacket::CombatHit {
            attacker_id: npc_id,
            target_id: player_id,
            damage,
            target_hp: new_hp,
            target_max_hp: player_max_hp,
        },
    );

    // Check for player death
    if new_hp == 0 {
        if let Some(mut player) = state.players.get_mut(&player_id) {
            player.position = Position {
                x: 3222,
                y: 3218,
                z: 0,
            };
            player.current_hp = player_max_hp;
            player.combat_target = None;
        }

        send_to_visible_players_from_pos(
            state,
            &player_pos,
            crate::net::ServerPacket::Death {
                entity_id: player_id,
                killer_id: Some(npc_id),
            },
        );
    }
}

fn send_to_visible_players_from_pos(
    state: &GameState,
    position: &Position,
    packet: crate::net::ServerPacket,
) {
    use crate::world::in_view_distance;
    for player in state.players.iter() {
        if in_view_distance(position, &player.position) {
            if let Some(sender) = &player.sender {
                let _ = sender.send(packet.clone());
            }
        }
    }
}

fn process_player(_player: &mut Player) {
    // Process queued actions
    // TODO: Combat, skilling, movement
}

fn save_all_players(state: &GameState) {
    for player in state.players.iter() {
        if let Err(e) = save_player(&player) {
            warn!("Failed to save player {}: {}", player.username, e);
        }
    }
}

// ============================================================================
// Data Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub username: String,
    pub position: Position,
    pub skills: Skills,
    pub inventory: Vec<Option<Item>>,
    pub equipment: Equipment,
    pub current_hp: u32,
    #[serde(skip)]
    pub combat_target: Option<CombatTarget>,
    #[serde(skip)]
    pub last_combat_tick: u32,
    #[serde(skip)]
    pub session: Option<PlayerSession>,
    #[serde(skip)]
    pub sender: Option<mpsc::UnboundedSender<crate::net::ServerPacket>>,
}

#[derive(Debug, Clone)]
pub enum CombatTarget {
    Player(u32),
    Npc(u32),
}

#[derive(Debug, Clone)]
pub struct PlayerSession {
    pub last_active: std::time::Instant,
}

impl Default for PlayerSession {
    fn default() -> Self {
        Self {
            last_active: std::time::Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Skills {
    pub attack: Skill,
    pub defence: Skill,
    pub strength: Skill,
    pub hitpoints: Skill,
    pub ranged: Skill,
    pub prayer: Skill,
    pub magic: Skill,
    pub cooking: Skill,
    pub woodcutting: Skill,
    pub fletching: Skill,
    pub fishing: Skill,
    pub firemaking: Skill,
    pub crafting: Skill,
    pub smithing: Skill,
    pub mining: Skill,
    pub herblore: Skill,
    pub agility: Skill,
    pub thieving: Skill,
    pub slayer: Skill,
    pub farming: Skill,
    pub runecrafting: Skill,
    pub hunter: Skill,
    pub construction: Skill,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Skill {
    pub level: u8,
    pub xp: u32,
}

impl Default for Skill {
    fn default() -> Self {
        Self { level: 1, xp: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Equipment {
    pub head: Option<Item>,
    pub cape: Option<Item>,
    pub amulet: Option<Item>,
    pub weapon: Option<Item>,
    pub body: Option<Item>,
    pub shield: Option<Item>,
    pub legs: Option<Item>,
    pub gloves: Option<Item>,
    pub boots: Option<Item>,
    pub ring: Option<Item>,
    pub ammo: Option<Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub amount: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemDef {
    pub id: u32,
    pub name: String,
    pub examine: String,
    pub stackable: bool,
    pub tradeable: bool,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDef {
    pub id: u32,
    pub name: String,
    pub combat_level: Option<u8>,
    pub examine: String,
    pub attack: u8,
    pub strength: u8,
    pub defence: u8,
    pub hitpoints: u8,
    pub is_aggressive: bool,
    pub aggro_range: u32,
    pub loot_table: Vec<LootDrop>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LootDrop {
    pub item_id: u32,
    pub min_amount: u32,
    pub max_amount: u32,
    pub chance: f32, // 0.0 to 1.0 (1.0 = 100% drop rate)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcSpawn {
    pub npc_id: u32,
    pub position: Position,
    pub wander_radius: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Npc {
    pub id: u32,
    pub def_id: u32,
    pub name: String,
    pub position: Position,
    pub spawn_position: Position,
    pub health: u32,
    pub max_health: u32,
    pub combat_target: Option<u32>, // Player ID being attacked
    pub last_combat_tick: u32,      // Last tick NPC attacked
    pub is_aggressive: bool,        // Does NPC auto-attack players?
    pub aggro_range: u32,           // Distance to detect players (if aggressive)
    pub respawn_tick: Option<u32>,  // Tick when NPC will respawn (if dead)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundItem {
    pub id: u32,      // Unique ground item ID
    pub item_id: u32, // Item definition ID
    pub amount: u32,  // Stack size
    pub position: Position,
    pub owner_id: Option<u32>, // If Some, only owner can pick up for 60 seconds
    pub spawn_tick: u32,       // When the item was dropped
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NpcDialogue {
    pub npc_id: u32,
    pub npc_name: String,
    pub dialogues: Vec<Dialogue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dialogue {
    pub id: u32,
    pub trigger: String,
    pub text: String,
    pub options: Vec<DialogueOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueOption {
    pub text: String,
    pub next_dialogue: Option<u32>,
}

// ============================================================================
// File I/O Helpers
// ============================================================================

/// Load JSON file or return default empty vec
fn load_json_or_default<T: for<'de> Deserialize<'de> + Default>(path: &str) -> T {
    match fs::read_to_string(path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|e| {
            warn!("Failed to parse {}: {}", path, e);
            T::default()
        }),
        Err(_) => {
            info!("File not found: {}, using defaults", path);
            T::default()
        }
    }
}

/// Load all JSON files in a directory and merge them
fn load_all_json_in_dir<T: for<'de> Deserialize<'de>>(dir: &str) -> Vec<T> {
    let mut results = Vec::new();

    let Ok(entries) = fs::read_dir(dir) else {
        info!("Directory not found: {}", dir);
        return results;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Ok(contents) = fs::read_to_string(&path) {
                match serde_json::from_str::<Vec<T>>(&contents) {
                    Ok(mut items) => results.append(&mut items),
                    Err(e) => warn!("Failed to parse {:?}: {}", path, e),
                }
            }
        }
    }

    results
}

/// Save a player to their JSON file
pub fn save_player(player: &Player) -> std::io::Result<()> {
    let path = format!("data/players/{}.json", player.username.to_lowercase());
    let json = serde_json::to_string_pretty(player)?;
    fs::write(path, json)
}

/// Load a player from their JSON file
pub fn load_player(username: &str) -> Option<Player> {
    let path = format!("data/players/{}.json", username.to_lowercase());
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}

impl Player {
    /// Create a new player with default stats
    pub fn new(id: u32, username: String) -> Self {
        let mut skills = Skills::default();
        skills.hitpoints = Skill {
            level: 10,
            xp: 1154,
        }; // Start with 10 HP

        // Calculate max HP before moving skills
        let max_hp = skills.hitpoints.level as u32 * 10;

        // Create starter inventory
        let mut inventory = vec![None; 28];
        inventory[0] = Some(Item {
            id: 1277, // Bronze sword
            amount: 1,
        });
        inventory[1] = Some(Item {
            id: 1511, // Logs
            amount: 10,
        });
        inventory[2] = Some(Item {
            id: 1, // Coins
            amount: 25,
        });

        Self {
            id,
            username,
            position: Position {
                x: 3222,
                y: 3218,
                z: 0,
            }, // Lumbridge
            skills,
            inventory,
            equipment: Equipment::default(),
            current_hp: max_hp, // Start at full HP
            combat_target: None,
            last_combat_tick: 0,
            session: Some(PlayerSession::default()),
            sender: None,
        }
    }
}

// ============================================================================
// Combat System
// ============================================================================

/// Calculate if an attack hits
/// Returns true if attack hits, false if it misses
pub fn calculate_hit(attacker_attack_level: u8, target_defence_level: u8) -> bool {
    use rand::Rng;

    // Simple formula: higher attack vs lower defence = better hit chance
    // Base 50% chance, modified by level difference
    let level_diff = attacker_attack_level as i32 - target_defence_level as i32;
    let hit_chance = 50.0 + (level_diff as f32 * 2.0);
    let hit_chance = hit_chance.clamp(10.0, 90.0); // Min 10%, max 90%

    let roll = rand::thread_rng().gen_range(0.0..100.0);
    roll < hit_chance
}

/// Calculate damage dealt by an attack
/// Returns damage amount (0 if miss)
pub fn calculate_damage(attacker_strength_level: u8, weapon_bonus: u32, hit: bool) -> u32 {
    if !hit {
        return 0;
    }

    use rand::Rng;

    // Base damage from strength level
    let base_damage = (attacker_strength_level as u32 + weapon_bonus) / 10;
    let max_hit = base_damage.max(1);

    // Random damage from 1 to max_hit
    rand::thread_rng().gen_range(1..=max_hit)
}

/// Get max HP for a player based on hitpoints level
pub fn get_max_hp(hitpoints_level: u8) -> u32 {
    hitpoints_level as u32 * 10
}

/// Check if entity is in combat range (1 tile)
pub fn in_combat_range(pos1: &Position, pos2: &Position) -> bool {
    if pos1.z != pos2.z {
        return false;
    }

    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();

    dx <= 1 && dy <= 1
}

/// Calculate XP required for a given level (OSRS formula)
pub fn xp_for_level(level: u8) -> u32 {
    if level <= 1 {
        return 0;
    }

    let mut total_xp = 0.0;
    for lvl in 1..level {
        let lvl_f = lvl as f64;
        total_xp += (lvl_f + 300.0 * 2_f64.powf(lvl_f / 7.0)).floor() / 4.0;
    }

    total_xp.floor() as u32
}

/// Calculate level for a given XP amount
pub fn level_for_xp(xp: u32) -> u8 {
    if xp == 0 {
        return 1;
    }

    // Binary search for efficiency
    let mut level = 1;
    while level < 99 && xp_for_level(level + 1) <= xp {
        level += 1;
    }

    level
}

/// Add XP to a skill and return the new level (returns Some(new_level) if leveled up)
pub fn add_xp(skill: &mut Skill, xp_gained: u32) -> Option<u8> {
    let old_level = skill.level;
    skill.xp = skill.xp.saturating_add(xp_gained);
    let new_level = level_for_xp(skill.xp);

    if new_level > old_level {
        skill.level = new_level;
        Some(new_level)
    } else {
        None
    }
}

/// Calculate combat XP gained from damage dealt (damage * 4 for hitpoints, attack, strength, defence)
pub fn calculate_combat_xp(damage: u32) -> u32 {
    damage * 4
}

/// Generate loot drops based on NPC's loot table
pub fn generate_loot(loot_table: &[LootDrop]) -> Vec<(u32, u32)> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut drops = Vec::new();

    for drop in loot_table {
        let roll: f32 = rng.gen();
        if roll < drop.chance {
            let amount = if drop.min_amount == drop.max_amount {
                drop.min_amount
            } else {
                rng.gen_range(drop.min_amount..=drop.max_amount)
            };
            drops.push((drop.item_id, amount));
        }
    }

    drops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_definitions_have_examine_text() {
        let state = GameState::load_from_files();

        // Skip if no items loaded
        if state.items.is_empty() {
            println!("Skipping test - no items loaded");
            return;
        }

        // All items should have examine text
        for item in &state.items {
            assert!(
                !item.examine.is_empty(),
                "Item {} should have examine text",
                item.name
            );
        }
    }

    #[test]
    fn test_npc_definitions_have_examine_text() {
        let state = GameState::load_from_files();

        // Skip if no NPCs loaded
        if state.npc_defs.is_empty() {
            println!("Skipping test - no NPCs loaded");
            return;
        }

        // All NPCs should have examine text
        for npc in &state.npc_defs {
            assert!(
                !npc.examine.is_empty(),
                "NPC {} should have examine text",
                npc.name
            );
        }
    }

    #[test]
    fn test_can_find_item_examine_by_id() {
        let state = GameState::load_from_files();

        // Skip if no items loaded
        if state.items.is_empty() {
            println!("Skipping test - no items loaded");
            return;
        }

        // Test finding item by ID (Coins = id 1)
        let coins = state.items.iter().find(|i| i.id == 1);
        assert!(coins.is_some(), "Should find Coins (id 1)");

        if let Some(item) = coins {
            assert_eq!(item.name, "Coins");
            assert_eq!(item.examine, "Lovely money!");
        }
    }

    #[test]
    fn test_can_find_npc_examine_by_def_id() {
        let state = GameState::load_from_files();

        // Skip if no NPCs loaded
        if state.npc_defs.is_empty() {
            println!("Skipping test - no NPCs loaded");
            return;
        }

        // Test finding NPC by def_id (Hans = id 3)
        let hans = state.npc_defs.iter().find(|n| n.id == 3);
        assert!(hans.is_some(), "Should find Hans (id 3)");

        if let Some(npc) = hans {
            assert_eq!(npc.name, "Hans");
            assert_eq!(npc.examine, "Walks around aimlessly.");
        }
    }

    #[test]
    fn test_item_examine_text_variety() {
        let state = GameState::load_from_files();

        // Skip if insufficient items loaded
        if state.items.len() < 3 {
            println!("Skipping test - not enough items loaded");
            return;
        }

        // Check that different items have different examine text
        let examine_texts: Vec<String> = state
            .items
            .iter()
            .take(5)
            .map(|i| i.examine.clone())
            .collect();

        // At least some variety in examine text
        let unique_count = examine_texts
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert!(unique_count >= 2, "Should have variety in examine text");
    }

    #[test]
    fn test_npc_with_combat_level() {
        let state = GameState::load_from_files();

        // Skip if no NPCs loaded
        if state.npc_defs.is_empty() {
            println!("Skipping test - no NPCs loaded");
            return;
        }

        // Find an NPC with a combat level (Guard = id 81, combat 21)
        let guard = state.npc_defs.iter().find(|n| n.id == 81);

        if let Some(npc) = guard {
            assert_eq!(npc.name, "Guard");
            assert_eq!(npc.combat_level, Some(21));
            assert!(!npc.examine.is_empty());
        }
    }

    #[test]
    fn test_npc_without_combat_level() {
        let state = GameState::load_from_files();

        // Skip if no NPCs loaded
        if state.npc_defs.is_empty() {
            println!("Skipping test - no NPCs loaded");
            return;
        }

        // Find an NPC without a combat level (Hans = id 3, no combat)
        let hans = state.npc_defs.iter().find(|n| n.id == 3);

        if let Some(npc) = hans {
            assert_eq!(npc.name, "Hans");
            assert_eq!(npc.combat_level, None);
            assert!(!npc.examine.is_empty());
        }
    }

    #[test]
    fn test_starter_inventory_items_have_definitions() {
        let state = GameState::load_from_files();

        // Skip if no items loaded
        if state.items.is_empty() {
            println!("Skipping test - no items loaded");
            return;
        }

        let player = Player::new(999, "TestPlayer".to_string());

        // Check that starter items exist in definitions
        for slot in &player.inventory {
            if let Some(item) = slot {
                let item_def = state.items.iter().find(|i| i.id == item.id);
                assert!(
                    item_def.is_some(),
                    "Starter item {} should have a definition",
                    item.id
                );
            }
        }
    }

    fn test_combat_hit_calculation() {
        // Equal levels should have ~50% hit chance
        let mut hits = 0;
        for _ in 0..100 {
            if calculate_hit(10, 10) {
                hits += 1;
            }
        }
        // Should be around 50 hits (allow 30-70 range for randomness)
        assert!(hits >= 30 && hits <= 70, "Hit rate should be around 50%");
    }

    #[test]
    fn test_combat_hit_advantage() {
        // Higher attack should have better hit chance
        let mut high_attack_hits = 0;
        let mut low_attack_hits = 0;

        for _ in 0..100 {
            if calculate_hit(50, 10) {
                high_attack_hits += 1;
            }
            if calculate_hit(10, 50) {
                low_attack_hits += 1;
            }
        }

        // Higher attack should hit more often
        assert!(
            high_attack_hits > low_attack_hits,
            "Higher attack level should hit more often"
        );
    }

    #[test]
    fn test_combat_damage_on_miss() {
        // Miss should deal 0 damage
        let damage = calculate_damage(50, 10, false);
        assert_eq!(damage, 0, "Missed attacks should deal no damage");
    }

    #[test]
    fn test_combat_damage_on_hit() {
        // Hit should deal at least 1 damage
        let damage = calculate_damage(50, 10, true);
        assert!(damage > 0, "Hits should deal at least 1 damage");
    }

    #[test]
    fn test_combat_damage_scales_with_strength() {
        // Higher strength should have potential for higher damage
        let mut high_str_total = 0;
        let mut low_str_total = 0;

        for _ in 0..100 {
            high_str_total += calculate_damage(50, 0, true);
            low_str_total += calculate_damage(10, 0, true);
        }

        let high_str_avg = high_str_total / 100;
        let low_str_avg = low_str_total / 100;

        assert!(
            high_str_avg > low_str_avg,
            "Higher strength should deal more damage on average"
        );
    }

    #[test]
    fn test_max_hp_calculation() {
        assert_eq!(get_max_hp(10), 100, "Level 10 HP should be 100");
        assert_eq!(get_max_hp(99), 990, "Level 99 HP should be 990");
        assert_eq!(get_max_hp(1), 10, "Level 1 HP should be 10");
    }

    #[test]
    fn test_combat_range_adjacent() {
        let pos1 = Position {
            x: 3200,
            y: 3200,
            z: 0,
        };

        // Adjacent tiles (within 1 tile)
        let adjacent = Position {
            x: 3201,
            y: 3200,
            z: 0,
        };
        assert!(
            in_combat_range(&pos1, &adjacent),
            "Should be in combat range"
        );

        // Diagonal adjacent
        let diagonal = Position {
            x: 3201,
            y: 3201,
            z: 0,
        };
        assert!(
            in_combat_range(&pos1, &diagonal),
            "Diagonal adjacent should be in combat range"
        );

        // Same tile
        assert!(
            in_combat_range(&pos1, &pos1),
            "Same tile should be in combat range"
        );
    }

    #[test]
    fn test_combat_range_too_far() {
        let pos1 = Position {
            x: 3200,
            y: 3200,
            z: 0,
        };

        // 2 tiles away
        let far = Position {
            x: 3202,
            y: 3200,
            z: 0,
        };
        assert!(
            !in_combat_range(&pos1, &far),
            "2 tiles away should be out of combat range"
        );

        // Way too far
        let very_far = Position {
            x: 3210,
            y: 3210,
            z: 0,
        };
        assert!(
            !in_combat_range(&pos1, &very_far),
            "Distant tiles should be out of combat range"
        );
    }

    #[test]
    fn test_combat_range_different_plane() {
        let ground = Position {
            x: 3200,
            y: 3200,
            z: 0,
        };

        let upstairs = Position {
            x: 3200,
            y: 3200,
            z: 1,
        };

        assert!(
            !in_combat_range(&ground, &upstairs),
            "Different planes should not be in combat range"
        );
    }

    #[test]
    fn test_player_starts_with_full_hp() {
        let player = Player::new(1, "TestPlayer".to_string());
        let max_hp = get_max_hp(player.skills.hitpoints.level);

        assert_eq!(
            player.current_hp, max_hp,
            "New players should start at full HP"
        );
        assert_eq!(player.current_hp, 100, "Level 10 HP should be 100");
    }

    #[test]
    fn test_combat_target_none_by_default() {
        let player = Player::new(1, "TestPlayer".to_string());

        assert!(
            player.combat_target.is_none(),
            "New players should have no combat target"
        );
        assert_eq!(
            player.last_combat_tick, 0,
            "New players should have no combat history"
        );
    }

    #[test]
    fn test_xp_for_level_1() {
        assert_eq!(xp_for_level(1), 0, "Level 1 should require 0 XP");
    }

    #[test]
    fn test_xp_for_level_2() {
        let xp = xp_for_level(2);
        assert_eq!(xp, 83, "Level 2 should require 83 XP");
    }

    #[test]
    fn test_xp_for_level_10() {
        let xp = xp_for_level(10);
        assert_eq!(xp, 1154, "Level 10 should require 1154 XP");
    }

    #[test]
    fn test_xp_for_level_50() {
        let xp = xp_for_level(50);
        assert_eq!(xp, 101333, "Level 50 should require 101333 XP");
    }

    #[test]
    fn test_xp_for_level_99() {
        let xp = xp_for_level(99);
        assert_eq!(xp, 13034431, "Level 99 should require 13034431 XP");
    }

    #[test]
    fn test_level_for_xp_zero() {
        assert_eq!(level_for_xp(0), 1, "0 XP should be level 1");
    }

    #[test]
    fn test_level_for_xp_level_2() {
        assert_eq!(level_for_xp(83), 2, "83 XP should be level 2");
        assert_eq!(level_for_xp(100), 2, "100 XP should be level 2");
    }

    #[test]
    fn test_level_for_xp_level_10() {
        assert_eq!(level_for_xp(1154), 10, "1154 XP should be level 10");
        assert_eq!(level_for_xp(1200), 10, "1200 XP should be level 10");
    }

    #[test]
    fn test_level_for_xp_level_50() {
        assert_eq!(level_for_xp(101333), 50, "101333 XP should be level 50");
    }

    #[test]
    fn test_level_for_xp_max() {
        let max_xp = xp_for_level(99);
        assert_eq!(
            level_for_xp(max_xp),
            99,
            "XP for level 99 should give level 99"
        );
        assert_eq!(level_for_xp(999999999), 99, "Excessive XP should cap at 99");
    }

    #[test]
    fn test_add_xp_no_level_up() {
        let mut skill = Skill { level: 1, xp: 0 };
        let result = add_xp(&mut skill, 50);

        assert_eq!(result, None, "Should not level up with 50 XP");
        assert_eq!(skill.xp, 50, "XP should be added");
        assert_eq!(skill.level, 1, "Level should remain 1");
    }

    #[test]
    fn test_add_xp_level_up() {
        let mut skill = Skill { level: 1, xp: 0 };
        let result = add_xp(&mut skill, 83);

        assert_eq!(result, Some(2), "Should level up to 2");
        assert_eq!(skill.xp, 83, "XP should be 83");
        assert_eq!(skill.level, 2, "Level should be 2");
    }

    #[test]
    fn test_add_xp_multiple_levels() {
        let mut skill = Skill { level: 1, xp: 0 };
        let result = add_xp(&mut skill, 1154);

        assert_eq!(result, Some(10), "Should level up to 10");
        assert_eq!(skill.xp, 1154, "XP should be 1154");
        assert_eq!(skill.level, 10, "Level should be 10");
    }

    #[test]
    fn test_add_xp_incremental() {
        let mut skill = Skill { level: 1, xp: 0 };

        // Add XP but not enough to level
        let result1 = add_xp(&mut skill, 50);
        assert_eq!(result1, None);
        assert_eq!(skill.level, 1);

        // Add more to reach level 2
        let result2 = add_xp(&mut skill, 33);
        assert_eq!(result2, Some(2));
        assert_eq!(skill.level, 2);
        assert_eq!(skill.xp, 83);
    }

    #[test]
    fn test_add_xp_overflow_protection() {
        let mut skill = Skill {
            level: 99,
            xp: u32::MAX - 10,
        };
        let result = add_xp(&mut skill, 20);

        assert_eq!(result, None, "Should not level past 99");
        assert_eq!(skill.xp, u32::MAX, "XP should saturate at max");
        assert_eq!(skill.level, 99, "Level should remain 99");
    }

    #[test]
    fn test_calculate_combat_xp() {
        assert_eq!(calculate_combat_xp(0), 0, "0 damage = 0 XP");
        assert_eq!(calculate_combat_xp(1), 4, "1 damage = 4 XP");
        assert_eq!(calculate_combat_xp(5), 20, "5 damage = 20 XP");
        assert_eq!(calculate_combat_xp(10), 40, "10 damage = 40 XP");
        assert_eq!(calculate_combat_xp(100), 400, "100 damage = 400 XP");
    }

    #[test]
    fn test_xp_system_integration() {
        let mut skill = Skill { level: 1, xp: 0 };

        // Simulate combat: deal 5 damage
        let damage = 5;
        let xp_gained = calculate_combat_xp(damage);
        assert_eq!(xp_gained, 20);

        // Add XP
        let result = add_xp(&mut skill, xp_gained);
        assert_eq!(result, None, "20 XP not enough for level 2");
        assert_eq!(skill.xp, 20);

        // Deal more damage
        let xp_gained2 = calculate_combat_xp(20); // 80 XP
        let result2 = add_xp(&mut skill, xp_gained2);
        assert_eq!(result2, Some(2), "100 total XP should level to 2");
        assert_eq!(skill.level, 2);
        assert_eq!(skill.xp, 100);
    }

    #[test]
    fn test_generate_loot_empty_table() {
        let loot_table: Vec<LootDrop> = vec![];
        let loot = generate_loot(&loot_table);
        assert_eq!(loot.len(), 0, "Empty loot table should drop nothing");
    }

    #[test]
    fn test_generate_loot_guaranteed_drop() {
        let loot_table = vec![LootDrop {
            item_id: 995,
            min_amount: 10,
            max_amount: 10,
            chance: 1.0,
        }];

        let loot = generate_loot(&loot_table);
        assert_eq!(loot.len(), 1, "Should have 1 guaranteed drop");
        assert_eq!(loot[0].0, 995, "Should drop item 995");
        assert_eq!(loot[0].1, 10, "Should drop exactly 10");
    }

    #[test]
    fn test_generate_loot_zero_chance() {
        let loot_table = vec![LootDrop {
            item_id: 995,
            min_amount: 1,
            max_amount: 1,
            chance: 0.0,
        }];

        let loot = generate_loot(&loot_table);
        assert_eq!(loot.len(), 0, "0% chance should never drop");
    }

    #[test]
    fn test_generate_loot_multiple_drops() {
        let loot_table = vec![
            LootDrop {
                item_id: 995,
                min_amount: 1,
                max_amount: 1,
                chance: 1.0,
            },
            LootDrop {
                item_id: 526,
                min_amount: 1,
                max_amount: 1,
                chance: 1.0,
            },
        ];

        let loot = generate_loot(&loot_table);
        assert_eq!(loot.len(), 2, "Should have 2 guaranteed drops");
        assert!(loot.iter().any(|(id, _)| *id == 995));
        assert!(loot.iter().any(|(id, _)| *id == 526));
    }

    #[test]
    fn test_generate_loot_random_amount() {
        let loot_table = vec![LootDrop {
            item_id: 995,
            min_amount: 1,
            max_amount: 100,
            chance: 1.0,
        }];

        let loot = generate_loot(&loot_table);
        assert_eq!(loot.len(), 1);
        let amount = loot[0].1;
        assert!(
            amount >= 1 && amount <= 100,
            "Amount should be between 1 and 100, got {}",
            amount
        );
    }

    #[test]
    fn test_npc_aggressive_flag() {
        let npc = Npc {
            id: 1,
            def_id: 198,
            name: "Goblin".to_string(),
            position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            spawn_position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            health: 50,
            max_health: 50,
            combat_target: None,
            last_combat_tick: 0,
            is_aggressive: true,
            aggro_range: 5,
            respawn_tick: None,
        };

        assert!(npc.is_aggressive, "Goblin should be aggressive");
        assert_eq!(npc.aggro_range, 5, "Goblin aggro range should be 5");
    }

    #[test]
    fn test_npc_combat_target() {
        let mut npc = Npc {
            id: 1,
            def_id: 198,
            name: "Goblin".to_string(),
            position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            spawn_position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            health: 50,
            max_health: 50,
            combat_target: None,
            last_combat_tick: 0,
            is_aggressive: true,
            aggro_range: 5,
            respawn_tick: None,
        };

        assert!(
            npc.combat_target.is_none(),
            "NPC should start with no target"
        );

        npc.combat_target = Some(42);
        assert_eq!(npc.combat_target, Some(42), "NPC should target player 42");
    }

    #[test]
    fn test_npc_respawn_tick() {
        let mut npc = Npc {
            id: 1,
            def_id: 198,
            name: "Goblin".to_string(),
            position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            spawn_position: Position {
                x: 3200,
                y: 3200,
                z: 0,
            },
            health: 0,
            max_health: 50,
            combat_target: None,
            last_combat_tick: 0,
            is_aggressive: true,
            aggro_range: 5,
            respawn_tick: None,
        };

        assert!(
            npc.respawn_tick.is_none(),
            "NPC should start with no respawn"
        );

        npc.respawn_tick = Some(100);
        assert_eq!(
            npc.respawn_tick,
            Some(100),
            "NPC should respawn at tick 100"
        );
    }

    #[test]
    fn test_loot_drop_structure() {
        let drop = LootDrop {
            item_id: 995,
            min_amount: 1,
            max_amount: 100,
            chance: 0.5,
        };

        assert_eq!(drop.item_id, 995);
        assert_eq!(drop.min_amount, 1);
        assert_eq!(drop.max_amount, 100);
        assert_eq!(drop.chance, 0.5);
    }

    #[test]
    fn test_npc_def_with_combat_stats() {
        let npc_def = NpcDef {
            id: 198,
            name: "Goblin".to_string(),
            combat_level: Some(5),
            examine: "An ugly green creature.".to_string(),
            attack: 5,
            strength: 5,
            defence: 3,
            hitpoints: 5,
            is_aggressive: true,
            aggro_range: 5,
            loot_table: vec![LootDrop {
                item_id: 995,
                min_amount: 5,
                max_amount: 20,
                chance: 0.6,
            }],
        };

        assert_eq!(npc_def.attack, 5);
        assert_eq!(npc_def.strength, 5);
        assert_eq!(npc_def.defence, 3);
        assert_eq!(npc_def.hitpoints, 5);
        assert_eq!(npc_def.is_aggressive, true);
        assert_eq!(npc_def.loot_table.len(), 1);
    }
}
