# 🦀 Rustscape

A simplified RuneScape-inspired MMORPG server written in Rust with WebSocket support for browser-based clients.

## Features

- **Single Binary**: No Docker, no PostgreSQL, no Redis - just `cargo run`
- **WebSocket Support**: Browser-based clients connect directly
- **File-Based Storage**: Players saved as JSON files
- **Hot-Reloadable Data**: Game definitions loaded from JSON files
- **Async Rust**: Built with Tokio and Axum for high performance
- **Combat System**: PvP and PvE combat with hit/miss calculation and damage
- **XP & Leveling**: OSRS-accurate XP formula with 23 skills (levels 1-99)
- **Ground Items**: Drop, pickup, and visibility system
- **NPC Dialogue**: Interactive dialogue with branching options
- **Examine System**: Examine items and NPCs for descriptions

## Quick Start

```bash
# 1. Run the server from project root
cargo run

# 2. Open browser to http://localhost:8080
```

That's it! No complex setup required.

## Project Structure

```
rustscape/
├── Cargo.toml                # Rust project configuration
├── src/
│   ├── main.rs              # Entry point, HTTP + WebSocket server
│   ├── game/mod.rs          # Game state, tick loop, data types
│   ├── net/mod.rs           # WebSocket handling, packets
│   └── world/mod.rs         # Regions, collision, visibility
├── assets/
│   ├── definitions/         # Game data (git tracked)
│   │   ├── items.json
│   │   └── npcs.json
│   ├── dialogue/            # NPC dialogue trees
│   │   └── npcs/
│   └── spawns/              # NPC/object spawn locations
│       └── npcs/
│           └── lumbridge.json
├── client/
│   ├── dist/                # Static web files served by server
│   │   ├── index.html       # Landing page (auto-redirects)
│   │   ├── game.html        # Modern game client
│   │   └── test-client.html # Debug/test client
│   └── README.md            # Client documentation
├── data/                    # Runtime data (.gitignored)
│   └── players/             # Player save files (JSON)
├── docs/                    # Documentation
├── scripts/                 # Utility scripts
├── LICENSE
├── run.sh                   # Quick run script
└── README.md                # This file
```

## How It Works

### Server Architecture
- **Axum**: Web framework handling HTTP + WebSocket
- **Tokio**: Async runtime for concurrent connections
- **Game Loop**: 600ms tick cycle for game updates
- **Broadcast**: Server-wide event system for chat, player updates

### Data Flow
```
Client                          Server
  │                               │
  ├─── Connect ────────────────►  │
  │                               │
  ├─── Login { username } ─────►  │
  │                               ├── Load/create player.json
  │  ◄── LoginSuccess { ... } ─── │
  │                               │
  ├─── Move { x, y } ──────────►  │
  │                               ├── Update position
  │  ◄── PlayerMoved (broadcast)  │
  │                               │
  ├─── Chat { message } ───────►  │
  │  ◄── ChatMessage (broadcast)  │
```

## Development

### Building the Server

```bash
cd src
cargo build --release
cargo test
cargo run --release
```

### Editing Game Data

Game definitions are in `src/assets/definitions/`:

**items.json**
```json
{
  "id": 1277,
  "name": "Bronze sword",
  "examine": "A bronze sword.",
  "stackable": false,
  "tradeable": true,
  "value": 26
}
```

**npcs.json**
```json
{
  "id": 3,
  "name": "Hans",
  "combat_level": null,
  "examine": "Walks around aimlessly."
}
```

### Adding NPC Spawns

Create/edit files in `src/assets/spawns/npcs/`:

```json
{
  "npc_id": 3,
  "position": { "x": 3222, "y": 3218, "z": 0 },
  "wander_radius": 5
}
```

### Player Data

Players are saved as JSON in `src/data/players/`:

```json
{
  "id": 1,
  "username": "TestPlayer",
  "position": { "x": 3222, "y": 3218, "z": 0 },
  "skills": {
    "attack": { "level": 1, "xp": 0 },
    "hitpoints": { "level": 10, "xp": 1154 }
  },
  "inventory": [null, null, ...],
  "equipment": { "head": null, "weapon": null, ... }
}
```

## Packet Reference

### Client → Server

| Type | Fields | Description |
|------|--------|-------------|
| `Login` | `username`, `password` | Authenticate |
| `Move` | `x`, `y` | Move player |
| `Chat` | `message` | Send chat |
| `RequestPlayers` | - | Get nearby players |
| `RequestNpcs` | - | Get nearby NPCs |
| `RequestGroundItems` | - | Get ground items |
| `DropItem` | `slot` | Drop inventory item |
| `PickupItem` | `ground_item_id` | Pickup ground item |
| `TalkToNpc` | `npc_id` | Start NPC dialogue |
| `SelectDialogueOption` | `npc_id`, `dialogue_id`, `option_index` | Choose dialogue option |
| `ExamineItem` | `item_id` | Examine item definition |
| `ExamineNpc` | `npc_id` | Examine NPC |
| `Attack` | `target_type`, `target_id` | Attack player/NPC |
| `Ping` | `timestamp` | Measure latency |

### Server → Client

| Type | Fields | Description |
|------|--------|-------------|
| `Welcome` | `message`, `tick` | Connection established |
| `LoginSuccess` | `player_id`, `position`, `skills` | Logged in |
| `PlayerEnter` | `id`, `username`, `position` | Player entered view |
| `PlayerLeft` | `id` | Player left view |
| `PlayerMoved` | `id`, `position` | Player moved (broadcast) |
| `PlayerList` | `players[]` | Nearby players |
| `NpcList` | `npcs[]` | Nearby NPCs |
| `ChatMessage` | `username`, `message` | Chat (broadcast) |
| `GroundItemSpawned` | `item` | Ground item appeared |
| `GroundItemRemoved` | `ground_item_id` | Ground item removed |
| `GroundItemList` | `items[]` | All ground items |
| `NpcDialogue` | `npc_id`, `npc_name`, `dialogue_id`, `text`, `options[]` | NPC dialogue |
| `ExamineText` | `text` | Examine description |
| `CombatHit` | `attacker_id`, `target_id`, `damage`, `target_hp`, `target_max_hp` | Combat hit |
| `Death` | `entity_id`, `killer_id` | Entity died |
| `HealthUpdate` | `entity_id`, `current_hp`, `max_hp` | HP changed |
| `XpGain` | `skill_name`, `xp_gained`, `total_xp` | XP gained |
| `LevelUp` | `skill_name`, `new_level` | Leveled up! |
| `Pong` | `timestamp`, `server_tick` | Ping response |

## Multiplayer Setup

### Option A: Port Forwarding
Forward port 8080 on your router to your machine.

### Option B: Tailscale (Recommended)
```bash
# Install Tailscale on all machines
tailscale up
# Share your Tailscale IP (100.x.x.x)
```

### Option C: ngrok
```bash
ngrok http 8080
# Share the ngrok URL
```

## Technology Stack

- **Rust** - High-performance, memory-safe server
- **Tokio** - Async runtime for networking
- **Axum** - Web framework (HTTP + WebSocket)
- **Serde** - JSON serialization
- **DashMap** - Concurrent HashMap for player state

## When to Scale Up

This simplified architecture is perfect for:
- Solo development
- Playing with 5-10 friends
- Prototyping and experimentation

You don't need Docker/PostgreSQL/Redis until:
- 50+ concurrent players
- Need horizontal scaling (multiple servers)
- Complex queries on player data
- Strict persistence guarantees

## Performance

The 600ms game tick can handle:
- ~100 concurrent players (single core)
- ~500+ with spatial partitioning
- Client-side interpolation makes movement smooth

See `todo.md` for optimization strategies.

## License

MIT License - See [LICENSE](LICENSE) for details.