# Rustscape - Project Context & Forward Plan

**Last Updated**: February 2024  
**Status**: Simplified Architecture - Development Ready  
**Goal**: Playable MMORPG for small groups (5-10 players)

---

## 🎯 Project Vision

Rustscape is a **2009-era RuneScape-inspired MMORPG** rebuilt from scratch in Rust. The focus is on creating a **simple, playable game** for you and your friends, not enterprise-scale infrastructure.

### Core Philosophy
- **Simplicity over scalability**: JSON files beat databases for <50 players
- **Single binary deployment**: One `cargo run` command to start
- **Browser-based client**: No downloads, just open a URL
- **Rapid iteration**: Edit JSON, restart server, test immediately

---

## 📐 Current Architecture

### The "No Infrastructure" Stack

```
┌─────────────────────────────────────────────┐
│         Browser (Client)                    │
│   - HTML/CSS/JavaScript                     │
│   - WebSocket connection                    │
│   - JSON packet protocol                    │
└──────────────┬──────────────────────────────┘
               │ WebSocket
               ↓
┌─────────────────────────────────────────────┐
│    Rustscape Server (Single Binary)         │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │  Axum Web Server (Port 8080)           │ │
│  │  - Serves static HTML/JS/CSS           │ │
│  │  - WebSocket endpoint (/ws)            │ │
│  │  - Status API (/api/status)            │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │  Game State (In-Memory)                │ │
│  │  - DashMap<PlayerId, Player>           │ │
│  │  - DashMap<NpcId, Npc>                 │ │
│  │  - Item/NPC definitions (Vec)          │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │  Game Loop (600ms tick)                │ │
│  │  - NPC AI/movement                     │ │
│  │  - Player actions                      │ │
│  │  - Auto-save (every 60 seconds)        │ │
│  └────────────────────────────────────────┘ │
│                                              │
│  ┌────────────────────────────────────────┐ │
│  │  Persistence (File System)             │ │
│  │  - assets/definitions/*.json (read)    │ │
│  │  - data/players/*.json (read/write)    │ │
│  └────────────────────────────────────────┘ │
└─────────────────────────────────────────────┘
```

### No Docker. No PostgreSQL. No Redis. No nginx.

**Just Rust + Tokio + Axum + JSON files.**

---

## 📁 Project Structure

```
rustscape/
├── src/                          # Main server directory
│   ├── src/                      # Rust source code
│   │   ├── main.rs               # Entry point (90 lines)
│   │   │                         # - Axum web server setup
│   │   │                         # - WebSocket handler
│   │   │                         # - Serves client/dist/ as static files
│   │   │
│   │   ├── game/mod.rs           # Game state & logic
│   │   │   ├── GameState         # Concurrent player/NPC storage
│   │   │   ├── tick_loop()       # 600ms game loop
│   │   │   ├── Player struct     # Position, skills, inventory
│   │   │   ├── Npc struct        # NPCs with AI
│   │   │   ├── ItemDef           # Item definitions
│   │   │   └── save/load logic   # JSON persistence
│   │   │
│   │   ├── net/mod.rs            # WebSocket & packets
│   │   │   ├── handle_connection() # Per-client handler
│   │   │   ├── ClientPacket      # Login, Move, Chat, etc.
│   │   │   └── ServerPacket      # LoginSuccess, PlayerMoved, etc.
│   │   │
│   │   └── world/mod.rs          # Collision & regions (stub)
│   │       ├── get_region_id()
│   │       └── can_move_to()
│   │
│   ├── assets/                   # Game data (git tracked)
│   │   ├── definitions/
│   │   │   ├── items.json        # All item definitions
│   │   │   └── npcs.json         # All NPC definitions
│   │   └── spawns/
│   │       └── npcs/
│   │           └── lumbridge.json # NPC spawn locations
│   │
│   ├── data/                     # Runtime data (.gitignored)
│   │   └── players/              # Player save files
│   │       └── username.json     # One file per player
│   │
│   ├── client/                   # Browser client
│   │   └── dist/
│   │       └── index.html        # Test client (7.8 KB)
│   │                             # - WebSocket connection
│   │                             # - JSON packet send/receive
│   │                             # - Chat, movement, debugging UI
│   │
│   ├── target/                   # Cargo build output
│   ├── Cargo.toml                # Dependencies
│   ├── Cargo.lock
│   ├── .gitignore
│   └── README.md                 # Quick start guide
│
├── docs/                         # Documentation
│   ├── PROJECT_CONTEXT.md        # ← YOU ARE HERE
│   ├── CLEANUP_SUMMARY.md        # What was removed
│   ├── QUICKSTART.md             # (needs update)
│   ├── rustscape_deployment_plan.md # (outdated)
│   └── todo.md                   # Optimization notes
│
├── scripts/
│   └── diagnose.sh               # Health check script (needs update)
│
├── LICENSE
└── README.md                     # Main project README
```

---

## 🔧 Technology Stack

### Server (Rust)
| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.x | Async runtime (full features) |
| `axum` | 0.7 | Web framework (HTTP + WebSocket) |
| `tower-http` | 0.5 | Static file serving, CORS |
| `serde` | 1.x | JSON serialization |
| `serde_json` | 1.x | JSON parsing |
| `tracing` | 0.1 | Logging framework |
| `tracing-subscriber` | 0.3 | Log output formatting |
| `dashmap` | 5.x | Concurrent HashMap (lock-free) |
| `bytes` | 1.x | Efficient byte buffers |
| `futures` | 0.3 | Async utilities |

### Client (Browser)
- Vanilla HTML/CSS/JavaScript
- WebSocket API
- JSON for packet serialization
- No build step required

### Data Format
- **JSON** for everything
- Human-readable and editable
- Git-friendly (easy diffs)
- No schema migrations needed

---

## 🎮 Game Features (Current State)

### ✅ Implemented
- [x] WebSocket server on port 8080
- [x] Static file serving (client files)
- [x] Player login/logout
- [x] Player movement (x, y coordinates)
- [x] NPC spawning from JSON files
- [x] Chat system (broadcast to all players)
- [x] Player persistence (JSON save files)
- [x] Auto-save (every 60 seconds)
- [x] Game tick loop (600ms cycle)
- [x] Item definitions (loaded from JSON)
- [x] NPC definitions (loaded from JSON)
- [x] Skills system (data structure)
- [x] Inventory system (28 slots)
- [x] Equipment system (slots defined)

### 🚧 Partially Implemented
- [ ] Collision detection (stub exists)
- [ ] Region system (code exists, not fully used)
- [ ] NPC AI/movement (placeholder)
- [ ] Player-NPC interaction
- [ ] Item pickup/drop
- [ ] Combat system
- [ ] Skill training

### 📋 Not Implemented (Future)
- [ ] Quests
- [ ] Trading
- [ ] Banking
- [ ] Friends list
- [ ] Clan chat
- [ ] Graphics beyond test UI

---

## 🔌 Protocol & Packets

### Client → Server (ClientPacket)

```rust
enum ClientPacket {
    Login { username: String, password: String },
    Move { x: i32, y: i32 },
    Chat { message: String },
    RequestPlayers,
    RequestNpcs,
    Ping { timestamp: u64 },
}
```

### Server → Client (ServerPacket)

```rust
enum ServerPacket {
    Welcome { message: String, tick: u32 },
    LoginSuccess { player_id: u32, position: Position, skills: HashMap<...> },
    LoginFailed { reason: String },
    PlayerMoved { id: u32, position: Position },
    PlayerList { players: Vec<PlayerInfo> },
    NpcList { npcs: Vec<NpcInfo> },
    ChatMessage { username: String, message: String },
    Pong { timestamp: u64, server_tick: u32 },
}
```

### Transport
- **Format**: JSON (text-based for now)
- **Future**: Could switch to binary for performance
- **WebSocket**: Text frames for JSON packets

---

## 💾 Data Persistence

### Player Save Format

**File**: `data/players/{username}.json`

```json
{
  "id": 1,
  "username": "TestPlayer",
  "password_hash": "...",
  "position": {
    "x": 3222,
    "y": 3218,
    "z": 0
  },
  "skills": {
    "attack": { "level": 1, "xp": 0 },
    "defence": { "level": 1, "xp": 0 },
    "strength": { "level": 1, "xp": 0 },
    "hitpoints": { "level": 10, "xp": 1154 },
    "ranged": { "level": 1, "xp": 0 },
    "prayer": { "level": 1, "xp": 0 },
    "magic": { "level": 1, "xp": 0 }
  },
  "inventory": [
    null,
    { "id": 1277, "amount": 1 },
    null,
    ...
  ],
  "equipment": {
    "head": null,
    "cape": null,
    "amulet": null,
    "weapon": null,
    "body": null,
    "shield": null,
    "legs": null,
    "gloves": null,
    "boots": null,
    "ring": null,
    "ammo": null
  }
}
```

### Game Definitions

**Items**: `assets/definitions/items.json`
```json
[
  {
    "id": 1277,
    "name": "Bronze sword",
    "examine": "A bronze sword.",
    "stackable": false,
    "tradeable": true,
    "value": 26
  }
]
```

**NPCs**: `assets/definitions/npcs.json`
```json
[
  {
    "id": 3,
    "name": "Hans",
    "combat_level": null,
    "examine": "Walks around aimlessly."
  }
]
```

**Spawns**: `assets/spawns/npcs/lumbridge.json`
```json
[
  {
    "npc_id": 3,
    "position": { "x": 3222, "y": 3218, "z": 0 },
    "wander_radius": 5
  }
]
```

---

## 🚀 How to Run (Current State)

### Development Mode
```bash
cd src
cargo run
```

Then open: http://localhost:8080

### Release Mode
```bash
cd src
cargo build --release
./target/release/rustscape
```

### What Happens
1. Server loads item definitions from `assets/definitions/items.json`
2. Server loads NPC definitions from `assets/definitions/npcs.json`
3. Server spawns NPCs from `assets/spawns/npcs/*.json`
4. Server starts game tick loop (600ms interval)
5. Server starts Axum web server on port 8080
6. Server serves `client/dist/index.html` as homepage
7. Server listens for WebSocket connections at `/ws`

### Test Client Features
- Connect with username
- Send login packet
- Receive player data
- Send chat messages
- Request player/NPC lists
- Move to random coordinates
- Ping/pong for latency testing

---

## 🎯 The Forward Plan

### Phase 1: Verify & Document (CURRENT PHASE)
**Goal**: Ensure everything compiles and runs cleanly.

#### Tasks
- [x] Remove old Docker/PostgreSQL/Redis setup
- [x] Consolidate assets into `src/assets/`
- [x] Verify directory structure
- [ ] Update all documentation to match reality
- [ ] Update `diagnose.sh` script
- [ ] Create this PROJECT_CONTEXT.md
- [ ] Write FORWARD_PLAN.md with detailed steps

#### Success Criteria
- `cargo run` starts server without errors
- Browser loads test client at http://localhost:8080
- WebSocket connects successfully
- Login packet exchange works
- No outdated documentation references

---

### Phase 2: Complete Core Gameplay Loop (NEXT)
**Goal**: Make the game actually playable.

#### 2.1 Movement & Collision
```rust
// In world/mod.rs
- [ ] Load collision maps from JSON
- [ ] Implement can_move_to() properly
- [ ] Add region-based visibility
- [ ] Test walking around Lumbridge
```

#### 2.2 NPC Interaction
```rust
// In game/mod.rs
- [ ] Add "Talk to NPC" packet
- [ ] NPC dialogue system (JSON-based)
- [ ] "Examine" for items and NPCs
```

#### 2.3 Item System
```rust
// In game/mod.rs
- [ ] Ground items (DashMap<Position, Vec<Item>>)
- [ ] Pickup item packet
- [ ] Drop item packet
- [ ] Item spawning
- [ ] Inventory management (add/remove)
```

#### 2.4 Client Upgrade
- [ ] Replace test client with proper UI
  - Option A: Build Kotlin/JS client you mentioned
  - Option B: Use simple Canvas/PixiJS renderer
  - Option C: Text-based MUD style (simplest)
- [ ] Render game world visually
- [ ] Click-to-walk movement
- [ ] Inventory UI
- [ ] Chat box

#### Success Criteria
- Can walk around without going through walls
- Can pick up items on ground
- Can see items in inventory
- Can talk to NPCs and get responses
- Can see other players move in real-time

---

### Phase 3: Combat & Skills (LATER)
**Goal**: Add progression systems.

#### 3.1 Combat System
- [ ] Attack player/NPC packet
- [ ] Combat tick processing (every 600ms)
- [ ] Damage calculation
- [ ] Death handling
- [ ] Respawn system

#### 3.2 Skill Training
- [ ] Woodcutting (click tree → get logs)
- [ ] Fishing (click spot → get fish)
- [ ] Mining (click rock → get ore)
- [ ] XP gain and level-up
- [ ] Skill requirements for items

#### Success Criteria
- Can kill an NPC and gain XP
- Can chop a tree and get logs
- Skills level up correctly
- Player death works without breaking game

---

### Phase 4: Multiplayer Deployment (FINAL)
**Goal**: Play with friends over internet.

#### 4.1 Choose Networking Method
- [ ] **Option A**: Tailscale (recommended)
  - Install on your PC and friends' PCs
  - Share Tailscale IP (100.x.x.x)
  - Friends connect to http://100.x.x.x:8080
  - Pros: Encrypted, easy, persistent IPs
  - Cons: Everyone needs to install

- [ ] **Option B**: ngrok (quick testing)
  - Run `ngrok http 8080`
  - Share the https://xxx.ngrok.io URL
  - Pros: Instant, no setup for friends
  - Cons: URL changes, bandwidth limits

- [ ] **Option C**: Port forwarding (traditional)
  - Forward port 8080 on your router
  - Share your public IP
  - Pros: Direct connection
  - Cons: Exposes network, needs static IP

#### 4.2 Create Deployment Package
```bash
#!/bin/bash
# deploy.sh - Package for distribution

mkdir -p rustscape-release
cp target/release/rustscape rustscape-release/
cp -r assets rustscape-release/
cp -r client rustscape-release/
mkdir rustscape-release/data

tar -czf rustscape.tar.gz rustscape-release/
```

#### 4.3 Player Guide
- [ ] Write HOW_TO_PLAY.md
- [ ] Create video/screenshots
- [ ] Document controls
- [ ] List features

#### Success Criteria
- Friend can connect from another network
- Multiple players see each other
- Chat works between players
- Game doesn't crash with 5+ players
- Data persists across server restarts

---

## 🛠️ Development Workflow

### Daily Development Loop
```bash
# 1. Edit game logic
vim src/src/game/mod.rs

# 2. Edit definitions
vim src/assets/definitions/npcs.json

# 3. Test immediately
cargo run
# Open http://localhost:8080

# 4. Check logs
# Server outputs to stdout with tracing

# 5. Debug
RUST_LOG=debug cargo run
```

### Adding New Content
```bash
# Add new item
echo '{"id": 1234, "name": "Dragon scimitar", ...}' >> assets/definitions/items.json

# Add new NPC
echo '{"id": 100, "name": "King", ...}' >> assets/definitions/npcs.json

# Spawn NPC in world
echo '{"npc_id": 100, "position": {...}}' >> assets/spawns/npcs/varrock.json

# Restart server (auto-reloads)
cargo run
```

### Backing Up Player Data
```bash
# Player saves are in data/players/
cp -r src/data/players src/data/players.backup.$(date +%Y%m%d)
```

---

## 📊 Performance Characteristics

### Current Limits (Single Server)
| Metric | Estimate | Notes |
|--------|----------|-------|
| Concurrent players | ~100 | Before tick lag |
| Players per region | ~50 | With spatial optimization |
| Tick duration | 600ms | Current target |
| Tick processing | <100ms | Leaves 500ms buffer |
| Memory per player | ~10 KB | In-memory state |
| Save file size | ~5 KB | JSON per player |

### Bottlenecks (Not Yet Reached)
1. **Tick loop**: If processing takes >600ms, game will lag
2. **Player updates**: Broadcasting to all players is O(n²)
3. **File I/O**: Auto-save can block if many players

### Optimizations (If Needed Later)
- Spatial partitioning (only update nearby players)
- Delta compression (only send what changed)
- Binary protocol (replace JSON with MessagePack)
- Async file saves (don't block game loop)
- SQLite for queries (if file system gets slow)

**For now: Don't optimize prematurely!**

---

## 🐛 Known Issues

### Current Bugs
- [ ] Collision detection not implemented (can walk through walls)
- [ ] NPC AI doesn't move them yet
- [ ] Player positions don't sync to other clients yet
- [ ] No authentication (password is ignored)
- [ ] No input validation (can crash with bad packets)

### Technical Debt
- [ ] Error handling is minimal (lots of `.unwrap()`)
- [ ] No tests written yet
- [ ] Hardcoded spawn position (3222, 3218)
- [ ] Client is a test UI, not a real game client
- [ ] No rate limiting (vulnerable to spam)

### Documentation Debt
- [ ] QUICKSTART.md references old architecture
- [ ] rustscape_deployment_plan.md is outdated
- [ ] diagnose.sh checks for wrong directory structure

---

## 🎓 Learning Resources

### Rust Game Development
- Bevy game engine (ECS architecture)
- Tokio async runtime docs
- Axum web framework guide

### RuneScape Protocol
- OSRS Wiki (game mechanics)
- Rune-server forums (protocol specs)
- OpenRS2 project (cache formats)

### Similar Projects
- OpenRune (OSRS server in Kotlin)
- Apollo (OSRS server in Java)
- RSMod (OSRS server in Kotlin)

---

## 🎯 Success Milestones

### Milestone 1: "It Runs" ✅
- Server compiles and starts
- Browser connects via WebSocket
- Can send/receive packets
- **STATUS**: ACHIEVED

### Milestone 2: "You Can Play" (In Progress)
- Can walk around a map
- Can talk to NPCs
- Can pick up items
- Can see other players
- **TARGET**: 2 weeks

### Milestone 3: "It's Fun" (Future)
- Can fight NPCs
- Can train skills
- Can complete quests
- Can trade with players
- **TARGET**: 2 months

### Milestone 4: "Friends Love It" (Future)
- Multiplayer works smoothly
- 5+ players online together
- No major bugs
- Content to explore
- **TARGET**: 3 months

---

## 📝 Immediate Next Actions

1. **Update Documentation** (1-2 hours)
   - [ ] Rewrite QUICKSTART.md for new architecture
   - [ ] Archive rustscape_deployment_plan.md
   - [ ] Update diagnose.sh script
   - [ ] Create FORWARD_PLAN.md with detailed tasks

2. **Verify Current State** (30 minutes)
   - [ ] Run `cargo build --release`
   - [ ] Test WebSocket connection
   - [ ] Verify login works
   - [ ] Check player save/load

3. **Plan Client Development** (1 hour)
   - [ ] Decide on client technology
   - [ ] Create mockup/wireframe
   - [ ] List required UI components
   - [ ] Choose rendering approach

4. **Implement Core Loop** (1-2 weeks)
   - [ ] Collision detection
   - [ ] NPC interaction
   - [ ] Item system
   - [ ] Basic client UI

---

## 💭 Philosophy & Principles

### Keep It Simple
- JSON files > Database until you hit real limits
- Single binary > Microservices for solo dev
- Vanilla JS > Framework bloat (for test client)
- File system > Redis for <100 players

### Make It Playable
- One playable feature > Ten half-done features
- Fun gameplay > Perfect code
- Friends playing > Optimal architecture

### Document Everything
- Future you will forget why you did things
- Comments explain "why", not "what"
- Keep docs updated or delete them

### Ship It
- Released and buggy > Perfect and unreleased
- Feedback from players > Personal assumptions
- Iterate fast > Plan forever

---

## 🎮 End Goal

**A RuneScape-inspired game that you and 5-10 friends can play together, with:**
- Working movement and collision
- NPCs to talk to and fight
- Items to collect and use
- Skills to train and level up
- A world to explore
- Multiplayer that actually works

**This is achievable in 3 months of focused development.**

---

*Last updated: February 2024*
*Next review: After Phase 2 completion*