//! Game state and tick loop

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::atomic::{AtomicU32, Ordering},
    time::Duration,
};
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

        let state = Self {
            players: DashMap::new(),
            items,
            npc_defs: npc_defs.clone(),
            npcs: DashMap::new(),
            next_id: AtomicU32::new(1),
            tick: AtomicU32::new(0),
        };

        // Spawn NPCs from spawn files
        for spawn in npc_spawns {
            let npc = Npc {
                id: state.next_id(),
                def_id: spawn.npc_id,
                name: npc_defs
                    .iter()
                    .find(|d| d.id == spawn.npc_id)
                    .map(|d| d.name.clone())
                    .unwrap_or_else(|| format!("NPC {}", spawn.npc_id)),
                position: spawn.position,
                spawn_position: spawn.position,
                health: 100,
                max_health: 100,
            };
            state.npcs.insert(npc.id, npc);
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

        // Process NPC movement/AI
        for mut npc in state.npcs.iter_mut() {
            process_npc(&mut npc);
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

fn process_npc(_npc: &mut Npc) {
    // Simple random wander AI
    // TODO: Implement actual NPC behavior
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
    #[serde(skip)]
    pub session: Option<PlayerSession>,
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

        Self {
            id,
            username,
            position: Position {
                x: 3222,
                y: 3218,
                z: 0,
            }, // Lumbridge
            skills,
            inventory: vec![None; 28],
            equipment: Equipment::default(),
            session: Some(PlayerSession::default()),
        }
    }
}
