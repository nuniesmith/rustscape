# 🦀 Rustscape

A simplified RuneScape-inspired MMORPG server written in Rust with WebSocket support for browser-based clients.

## Features

- **Single Binary**: No Docker, no PostgreSQL, no Redis - just `cargo run`
- **WebSocket Support**: Browser-based clients connect directly
- **File-Based Storage**: Players saved as JSON files
- **Hot-Reloadable Data**: Game definitions loaded from JSON files
- **Async Rust**: Built with Tokio and Axum for high performance

## Quick Start

```bash
# 1. Navigate to the source directory
cd src

# 2. Run the server
cargo run

# 3. Open browser to http://localhost:8080
```

That's it! No complex setup required.

## Project Structure

```
rustscape/
├── src/
│   ├── src/
│   │   ├── main.rs           # Entry point, HTTP + WebSocket server
│   │   ├── game/mod.rs       # Game state, tick loop, data types
│   │   ├── net/mod.rs        # WebSocket handling, packets
│   │   └── world/mod.rs      # Regions, collision (placeholder)
│   ├── assets/
│   │   ├── definitions/      # Game data (git tracked)
│   │   │   ├── items.json
│   │   │   └── npcs.json
│   │   └── spawns/           # NPC/object spawn locations
│   │       └── npcs/
│   │           └── lumbridge.json
│   ├── data/                 # Runtime data (.gitignored)
│   │   └── players/          # Player save files (JSON)
│   ├── client/
│   │   └── dist/             # Static files served to browser
│   │       └── index.html    # Test client
│   ├── Cargo.toml
│   ├── run.sh
│   └── README.md
├── LICENSE
└── README.md                 # This file
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
| `Ping` | `timestamp` | Measure latency |

### Server → Client

| Type | Fields | Description |
|------|--------|-------------|
| `Welcome` | `message`, `tick` | Connection established |
| `LoginSuccess` | `player_id`, `position`, `skills` | Logged in |
| `PlayerMoved` | `id`, `position` | Player moved (broadcast) |
| `PlayerList` | `players[]` | Nearby players |
| `NpcList` | `npcs[]` | Nearby NPCs |
| `ChatMessage` | `username`, `message` | Chat (broadcast) |
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