/**
 * Rustscape - Game Constants
 * Centralized configuration and constant values
 */

// ============================================================================
// GAME CONFIGURATION
// ============================================================================

export const GAME_CONFIG = {
    // Server connection
    SERVER_URL: "localhost",
    SERVER_PORT: 8080,
    WS_RECONNECT_DELAY: 3000,
    WS_MAX_RETRIES: 5,

    // Performance
    TARGET_FPS: 60,
    TICK_RATE: 600, // ms per game tick
    MAX_DELTA_TIME: 0.1, // Cap delta to prevent physics issues

    // Camera
    CAMERA_DISTANCE: 20,
    CAMERA_PITCH: 26.565, // Isometric angle in degrees
    CAMERA_MIN_ZOOM: 0.5,
    CAMERA_MAX_ZOOM: 3.0,
    CAMERA_ZOOM_SPEED: 0.1,

    // Movement
    WALK_SPEED: 1.5,
    RUN_SPEED: 3.0,
    MOVEMENT_SMOOTHING: 0.1,

    // World
    REGION_SIZE: 64,
    CHUNK_SIZE: 8,
    TILE_SIZE: 1.0,
    RENDER_DISTANCE: 100,
};

// ============================================================================
// CACHE CONFIGURATION
// ============================================================================

export const CACHE_CONFIG = {
    // Cache directory path
    BASE_PATH: "/assets/data_caches/560",

    // Archive types
    ARCHIVES: {
        ANIMATIONS: 0,
        SKELETONS: 1,
        SKINS: 2,
        INTERFACES: 3,
        SOUNDS: 4,
        MAPS: 5,
        MUSIC: 6,
        MODELS: 7,
        SPRITES: 8,
    },

    // Model IDs (Build 560)
    PLAYER_MODELS: {
        MALE: {
            HEAD: 0,
            TORSO: 18,
            ARMS: 26,
            LEGS: 36,
            FEET: 42,
            HANDS: 33,
        },
        FEMALE: {
            HEAD: 45,
            TORSO: 56,
            ARMS: 61,
            LEGS: 64,
            FEET: 79,
            HANDS: 65,
        },
    },

    // Animation IDs
    ANIMATIONS: {
        IDLE: 808,
        WALK: 819,
        RUN: 824,
        TURN_180: 820,
        TURN_90_CW: 821,
        TURN_90_CCW: 822,
        ATTACK: 422,
        DEFEND: 404,
        DEATH: 836,
    },
};

// ============================================================================
// UI CONSTANTS
// ============================================================================

export const UI_CONFIG = {
    // Sidebar
    SIDEBAR_WIDTH: 250,
    SIDEBAR_WIDTH_MOBILE: 200,

    // Chat
    CHAT_HEIGHT: 120,
    MAX_CHAT_MESSAGES: 100,
    CHAT_FADE_DELAY: 5000,

    // Inventory
    INVENTORY_ROWS: 7,
    INVENTORY_COLS: 4,
    INVENTORY_SIZE: 28,

    // Equipment slots
    EQUIPMENT_SLOTS: [
        "head",
        "cape",
        "neck",
        "ammo",
        "weapon",
        "body",
        "shield",
        "legs",
        "gloves",
        "boots",
        "ring",
    ],

    // Tab icons
    TAB_ICONS: {
        combat: "⚔️",
        stats: "📊",
        quest: "📜",
        inventory: "🎒",
        equipment: "👕",
        prayer: "🙏",
        spellbook: "✨",
    },
};

// ============================================================================
// COLOR CONSTANTS
// ============================================================================

export const COLORS = {
    // RuneScape Classic Theme
    RS_BROWN_DARKEST: 0x2e2822,
    RS_BROWN_DARK: 0x3e3529,
    RS_BROWN_MED: 0x524a3d,
    RS_BROWN_LIGHT: 0x6b614e,
    RS_STONE: 0x4e4a40,
    RS_GOLD: 0xd4a017,

    // Text colors (for Three.js)
    TEXT_YELLOW: 0xffff00,
    TEXT_WHITE: 0xffffff,
    TEXT_ORANGE: 0xff9040,
    TEXT_CYAN: 0x00ffff,
    TEXT_RED: 0xff0000,
    TEXT_GREEN: 0x00ff00,

    // Borders
    BORDER_LIGHT: 0x8b7355,
    BORDER_DARK: 0x1a1410,

    // XP Orbs
    HP_COLOR: 0xaa0000,
    PRAYER_COLOR: 0x0088aa,
    RUN_COLOR: 0xaaaa00,

    // Fog
    SKY_BLUE: 0x9eb4b8,
};

// ============================================================================
// NETWORK PACKET TYPES
// ============================================================================

export const PacketType = {
    // Client → Server
    LOGIN: "login",
    MOVE: "move",
    CHAT: "chat",
    COMMAND: "command",
    INTERACT_NPC: "interact_npc",
    INTERACT_OBJECT: "interact_object",
    USE_ITEM: "use_item",
    EQUIP: "equip",
    UNEQUIP: "unequip",
    DROP_ITEM: "drop_item",
    PICKUP_ITEM: "pickup_item",

    // Server → Client
    PLAYER_JOINED: "player_joined",
    PLAYER_LEFT: "player_left",
    PLAYER_MOVED: "player_moved",
    PLAYER_EQUIPMENT: "player_equipment",
    NPC_SPAWN: "npc_spawn",
    NPC_DESPAWN: "npc_despawn",
    NPC_MOVED: "npc_moved",
    CHAT_MESSAGE: "chat_message",
    GAME_STATE: "game_state",
    HEALTH_UPDATE: "health_update",
    XP_UPDATE: "xp_update",
    ITEM_SPAWNED: "item_spawned",
    ITEM_REMOVED: "item_removed",
};

// ============================================================================
// ENTITY CONSTANTS
// ============================================================================

export const ENTITY_CONFIG = {
    // Player
    PLAYER_HEIGHT: 2.0,
    PLAYER_RADIUS: 0.3,
    PLAYER_MASS: 1.0,

    // NPC
    NPC_UPDATE_DISTANCE: 50, // Only update NPCs within this distance
    NPC_DESPAWN_DISTANCE: 100, // Despawn NPCs beyond this distance
    NPC_WANDER_RADIUS: 5,
    NPC_WANDER_INTERVAL: 5000, // ms

    // Interaction
    INTERACT_DISTANCE: 2.0,
    ATTACK_DISTANCE: 1.5,
    TALK_DISTANCE: 2.0,

    // Name labels
    LABEL_HEIGHT_OFFSET: 2.2,
    LABEL_SCALE_WIDTH: 2.5,
    LABEL_SCALE_HEIGHT: 0.6,
};

// ============================================================================
// ANIMATION CONSTANTS
// ============================================================================

export const ANIMATION_CONFIG = {
    // Blend times (seconds)
    BLEND_IDLE_WALK: 0.2,
    BLEND_WALK_RUN: 0.15,
    BLEND_ANY_ATTACK: 0.1,

    // Loop settings
    LOOP_MODES: {
        ONCE: "once",
        REPEAT: "repeat",
        PING_PONG: "pingpong",
    },

    // Animation speeds
    SPEED_IDLE: 1.0,
    SPEED_WALK: 1.0,
    SPEED_RUN: 1.5,
    SPEED_ATTACK: 1.2,
};

// ============================================================================
// TERRAIN CONSTANTS
// ============================================================================

export const TERRAIN_CONFIG = {
    // Heightmap
    HEIGHT_SCALE: 0.1,
    MAX_HEIGHT: 10,
    MIN_HEIGHT: -10,

    // Textures
    TEXTURE_REPEAT: 4,
    TEXTURE_SIZE: 128,

    // LOD (Level of Detail)
    LOD_LEVELS: 3,
    LOD_DISTANCES: [20, 50, 100],

    // Collision
    COLLISION_STEP_HEIGHT: 0.5,
    COLLISION_SLOPE_LIMIT: 45, // degrees
};

// ============================================================================
// SKILL CONSTANTS
// ============================================================================

export const SKILLS = {
    ATTACK: 0,
    STRENGTH: 1,
    DEFENCE: 2,
    RANGED: 3,
    PRAYER: 4,
    MAGIC: 5,
    RUNECRAFTING: 6,
    HITPOINTS: 7,
    CRAFTING: 8,
    MINING: 9,
    SMITHING: 10,
    FISHING: 11,
    COOKING: 12,
    FIREMAKING: 13,
    WOODCUTTING: 14,
    AGILITY: 15,
    HERBLORE: 16,
    THIEVING: 17,
    FLETCHING: 18,
    SLAYER: 19,
    FARMING: 20,
    CONSTRUCTION: 21,
    HUNTER: 22,
};

export const SKILL_NAMES = [
    "Attack",
    "Strength",
    "Defence",
    "Ranged",
    "Prayer",
    "Magic",
    "Runecrafting",
    "Hitpoints",
    "Crafting",
    "Mining",
    "Smithing",
    "Fishing",
    "Cooking",
    "Firemaking",
    "Woodcutting",
    "Agility",
    "Herblore",
    "Thieving",
    "Fletching",
    "Slayer",
    "Farming",
    "Construction",
    "Hunter",
];

// ============================================================================
// DEBUG FLAGS
// ============================================================================

export const DEBUG = {
    SHOW_FPS: true,
    SHOW_HITBOXES: false,
    SHOW_GRID: false,
    SHOW_WIREFRAME: false,
    LOG_NETWORK: false,
    LOG_ANIMATIONS: false,
    LOG_CACHE_LOADING: true,
    SKIP_CACHE_VALIDATION: false,
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/**
 * Get XP required for a level
 * Uses OSRS formula
 */
export function getXPForLevel(level) {
    let xp = 0;
    for (let i = 1; i < level; i++) {
        xp += Math.floor(i + 300 * Math.pow(2, i / 7));
    }
    return Math.floor(xp / 4);
}

/**
 * Get level from XP
 */
export function getLevelFromXP(xp) {
    let level = 1;
    while (getXPForLevel(level + 1) <= xp && level < 99) {
        level++;
    }
    return level;
}

/**
 * Calculate combat level
 */
export function calculateCombatLevel(skills) {
    const attack = skills[SKILLS.ATTACK] || 1;
    const strength = skills[SKILLS.STRENGTH] || 1;
    const defence = skills[SKILLS.DEFENCE] || 1;
    const hitpoints = skills[SKILLS.HITPOINTS] || 10;
    const prayer = skills[SKILLS.PRAYER] || 1;
    const ranged = skills[SKILLS.RANGED] || 1;
    const magic = skills[SKILLS.MAGIC] || 1;

    const base = 0.25 * (defence + hitpoints + Math.floor(prayer / 2));
    const melee = 0.325 * (attack + strength);
    const range = 0.325 * (Math.floor(ranged / 2) + ranged);
    const mage = 0.325 * (Math.floor(magic / 2) + magic);

    return Math.floor(base + Math.max(melee, range, mage));
}
