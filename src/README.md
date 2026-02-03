# Rustscape (Simplified)

A simplified RuneScape-inspired game server for solo development.

## Quick Start

```bash
# 1. Create data directory
mkdir -p data/players

# 2. Run the server
cargo run

# 3. Open browser to http://localhost:8080
```

That's it. No Docker, no nginx, no PostgreSQL, no Redis.

## Project Structure

```
rustscape/
├── src/
│   ├── main.rs           # Entry point, HTTP + WebSocket server
│   ├── game/mod.rs       # Game state, tick loop, data types
│   ├── net/mod.rs        # WebSocket handling, packets
│   └── world/mod.rs      # Regions, collision (placeholder)
├── assets/
│   ├── definitions/      # Game data (git tracked)
│   │   ├── items.json
│   │   └── npcs.json
│   └── spawns/           # NPC/object spawn locations
│       └── npcs/
│           └── lumbridge.json
├── data/                 # Runtime data (.gitignored)
│   └── players/          # Player save files (JSON)
├── client/
│   └── dist/             # Static files served to browser
│       └── index.html    # Test client
├── Cargo.toml
├── run.sh
└── .gitignore
```

## How It Works

### Server
- Single Rust binary using Axum
- Serves static files + WebSocket on port 8080
- Game tick runs every 600ms
- Players saved as JSON files in `data/players/`

### Data Flow
```
Client                          Server
  │                               │
  ├─── Connect ────────────────►  │
  │                               │
  ├─── Login { username } ─────►  │
  │                               ├── Load player.json (or create new)
  │  ◄── LoginSuccess { pos } ─── │
  │                               │
  ├─── Move { x, y } ──────────►  │
  │                               ├── Update position
  │  ◄── PlayerMoved (broadcast)  │
  │                               │
  ├─── Chat { message } ───────►  │
  │  ◄── ChatMessage (broadcast)  │
  │                               │
  └─────────────────────────────  │
```

### Game Definitions
Edit these JSON files to change game data:

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

**spawns/npcs/lumbridge.json**
```json
{
  "npc_id": 3,
  "position": { "x": 3222, "y": 3218, "z": 0 },
  "wander_radius": 5
}
```

### Player Saves
Players are saved as JSON in `data/players/`:

```json
{
  "id": 1,
  "username": "TestPlayer",
  "position": { "x": 3222, "y": 3218, "z": 0 },
  "skills": {
    "attack": { "level": 1, "xp": 0 },
    "hitpoints": { "level": 10, "xp": 1154 },
    ...
  },
  "inventory": [null, null, ...],
  "equipment": { "head": null, "weapon": null, ... }
}
```

## Development Workflow

### Adding a new item
1. Edit `assets/definitions/items.json`
2. Restart server (or implement hot-reload)

### Adding NPC spawns
1. Create/edit file in `assets/spawns/npcs/`
2. Restart server

### Testing player changes
1. Edit `data/players/yourname.json` directly
2. Reconnect

### Sharing with friends
Option A: Port forward
```bash
# On your router, forward port 8080 to your machine
# Friends connect to http://your-public-ip:8080
```

Option B: Tailscale (easiest)
```bash
# Install Tailscale on your machine and friends' machines
tailscale up
# Share your Tailscale IP (100.x.x.x)
```

Option C: ngrok
```bash
ngrok http 8080
# Share the ngrok URL
```

## Integrating Your Existing Client

Replace `client/dist/` with your Kotlin/JS build output:

```bash
# In your client directory
./gradlew wasmJsBrowserDistribution

# Copy to server
cp -r build/dist/wasmJs/productionExecutable/* ../rustscape/client/dist/
```

Or just point your client at `ws://localhost:8080/ws`.

## When to Scale Up

You don't need Docker/PostgreSQL/Redis until:
- 50+ concurrent players
- Need horizontal scaling
- Need complex queries on player data
- Need persistence guarantees beyond "save every 60 seconds"

For 5 friends? JSON files are perfect.

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

## License

Do whatever you want with this.
