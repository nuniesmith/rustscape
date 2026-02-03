# Rustscape Performance Analysis & Optimization Guide

## Executive Summary

**Short answer: No, you don't need to build a custom "RuneTek-like" engine from scratch.**

What you've built is already architecturally sound. Your Rust server is an excellent foundation—Rust is already one of the highest-performance languages available. The real question is: what specific bottlenecks are you experiencing, and how can you optimize within your current architecture?

---

## Understanding RuneTek

RuneTek is Jagex's proprietary engine that has evolved through 7+ major versions:

| Version | Era | Key Features |
|---------|-----|--------------|
| RT1-RT2 | 1998-2001 | Basic 3D, 2D sprites |
| RT3 | 2003 | RS2 Standard Detail (still powers OSRS) |
| RT4 | 2008 | High Detail mode |
| RT5 | 2009 | Cross-platform (DirectX/OpenGL) |
| RT7/NXT | 2016 | C++ rewrite, modern rendering |

**Critical insight**: RuneTek is primarily a **client-side rendering engine**, not a server architecture. In 2016, Jagex rewrote the client from Java to C++ because they "hit saturation point" with Java's rendering limitations. The server-side logic remained largely the same.

Your situation is different—you have:
- A **Rust server** (already high-performance)
- A **Kotlin/JS web client** (browser-based, which has inherent constraints)

---

## Your Current Architecture Assessment

Based on your screenshots and codebase summary, you have:

### Strengths ✅
1. **Rust server** - Excellent choice for performance-critical game servers
2. **Binary protocol** - Your console shows raw opcodes (e.g., `opcode=134`, `opcode=253`), which is correct
3. **WebSocket transport** - Appropriate for browser games
4. **Redis** - Good for session management and pub/sub
5. **PostgreSQL** - Solid for persistent data
6. **Region-based world** - I see "Region: 2176, 4672" which indicates spatial partitioning

### Areas to Investigate 🔍
1. **Game tick implementation** - Is it a clean 600ms cycle?
2. **Player update protocol** - This is typically the #1 bottleneck
3. **Entity visibility** - How do you determine which entities a player can see?
4. **Packet batching** - Are you flushing once per tick or per packet?

---

## The 600ms Tick Challenge

RuneScape's 600ms tick is deceptively simple but hides complexity. Each tick must:

```
1. Process all incoming packets (movement, actions, chat)
2. Run game logic (combat, skills, NPC AI)
3. Update all entity states
4. Build and send player/NPC update packets
5. Flush all outgoing data
```

**The math gets scary fast:**

With 2,000 players, if each can see 255 others:
- 2,000 × 255 = 510,000 visibility checks per tick
- Add NPCs, ground items, projectiles...
- You're approaching 1M+ operations per 600ms

### Optimization Strategies

#### 1. Spatial Partitioning (Regions/Chunks)
```
You already have regions (2176, 4672). Ensure:
- Players only receive updates about entities in nearby regions
- Use a grid-based lookup (O(1)) instead of iterating all entities
- Cache region membership; update only on movement
```

#### 2. Player Update Protocol Optimization
```rust
// Bad: Rebuild appearance every tick
fn build_player_update(&self, observer: &Player) -> Vec<u8> {
    // Expensive full rebuild
}

// Good: Cache and invalidate
fn build_player_update(&self, observer: &Player) -> Vec<u8> {
    if !self.appearance_changed {
        return self.cached_appearance.clone();
    }
    // Rebuild and cache
}
```

#### 3. Delta Compression
Only send what changed:
```rust
struct PlayerUpdate {
    flags: u8,  // Bitmask of what changed
    // Only include fields that changed
    position: Option<Position>,
    animation: Option<u16>,
    health: Option<u8>,
}
```

#### 4. Packet Batching
```rust
// Bad: Flush after every packet
fn send_inventory_update(&mut self) {
    self.socket.write(&packet);
    self.socket.flush();  // Expensive!
}

// Good: Batch and flush once per tick
fn end_tick(&mut self) {
    self.socket.flush();  // Single flush
}
```

---

## Server Architecture Recommendations

### Consider ECS (Entity Component System)

Your current structure likely looks like this (OOP-style):
```rust
struct Player {
    position: Position,
    skills: Skills,
    inventory: Inventory,
    equipment: Equipment,
    // ... 50 more fields
}
```

ECS separates data from logic:
```rust
// Components (just data)
struct Position { x: i32, y: i32, z: i32 }
struct Health { current: u32, max: u32 }
struct Inventory { items: Vec<Option<Item>> }

// Systems (just logic)
fn movement_system(positions: &mut [Position], velocities: &[Velocity]) {
    // Process all moving entities in one cache-friendly loop
}
```

**Rust ECS Options:**
- **Bevy ECS** - Modern, parallel, excellent ergonomics
- **hecs** - Minimal, fast, no dependencies
- **specs** - Battle-tested, flexible

Benefits:
- Cache-friendly memory layout
- Easy parallelization with Rayon
- Clean separation of concerns

### Parallel Processing

Rust + Rayon makes this trivial:
```rust
use rayon::prelude::*;

// Process all players in parallel
players.par_iter_mut().for_each(|player| {
    player.process_tick();
});

// But be careful with shared state!
```

**Safe parallelization points:**
- NPC AI (independent per NPC)
- Pathfinding calculations
- Packet serialization
- Player update building (read-only world state)

**Must be sequential:**
- World state mutations
- Combat resolution (order matters)
- Item pickup (race conditions)

---

## Client-Side Optimization

Your Kotlin Multiplatform + Compose web client has different constraints than a native client.

### Browser Limitations
- **No UDP** - WebSocket is TCP-only; accept ~50-100ms additional latency
- **No multithreading** - WebWorkers help but aren't true threads
- **JavaScript performance** - Kotlin/JS compiles to JS, adding overhead
- **WebGL context limits** - Draw calls are expensive

### Client Optimization Strategies

#### 1. Interpolation (Essential for 600ms ticks)
```kotlin
// Don't teleport entities; interpolate between known positions
fun render(entity: Entity, deltaTime: Float) {
    val interpolated = lerp(
        entity.previousPosition,
        entity.targetPosition,
        min(1f, timeSinceUpdate / 600f)
    )
    draw(entity, interpolated)
}
```

#### 2. Client-Side Prediction
```kotlin
// Predict movement locally, reconcile with server
fun onMoveInput(direction: Direction) {
    // Immediately show movement
    localPlayer.predictedPosition = calculateMove(direction)
    
    // Send to server
    sendMovePacket(direction)
    
    // When server confirms, reconcile any difference
}
```

#### 3. Sprite Batching
```kotlin
// Bad: One draw call per sprite
sprites.forEach { sprite ->
    gl.bindTexture(sprite.texture)
    gl.drawArrays(...)
}

// Good: Batch by texture atlas
val batch = SpriteBatch()
sprites
    .groupBy { it.atlasId }
    .forEach { (atlas, sprites) ->
        batch.begin(atlas)
        sprites.forEach { batch.draw(it) }
        batch.end()  // Single draw call per atlas
    }
```

#### 4. Use Web Workers for Heavy Lifting
```kotlin
// Offload pathfinding, packet parsing to workers
val pathfindingWorker = Worker("pathfinding.js")
pathfindingWorker.postMessage(PathfindRequest(start, end))
pathfindingWorker.onmessage = { path ->
    applyPath(path)
}
```

---

## Recommended Architecture Evolution

### Phase 1: Measure First
Before optimizing, instrument your server:

```rust
use std::time::Instant;

fn game_tick(&mut self) {
    let tick_start = Instant::now();
    
    let t1 = Instant::now();
    self.process_packets();
    log::debug!("Packets: {:?}", t1.elapsed());
    
    let t2 = Instant::now();
    self.update_npcs();
    log::debug!("NPCs: {:?}", t2.elapsed());
    
    let t3 = Instant::now();
    self.build_player_updates();
    log::debug!("Player updates: {:?}", t3.elapsed());
    
    let t4 = Instant::now();
    self.flush_all();
    log::debug!("Flush: {:?}", t4.elapsed());
    
    let total = tick_start.elapsed();
    if total.as_millis() > 600 {
        log::warn!("Tick overrun: {:?}", total);
    }
}
```

### Phase 2: Optimize Bottlenecks
Based on measurements:

| If slow... | Optimize... |
|------------|-------------|
| Packet processing | Binary protocol, zero-copy parsing |
| NPC updates | Spatial partitioning, parallel AI |
| Player updates | Caching, delta compression |
| Flush | Single flush per tick, TCP_NODELAY |

### Phase 3: Scale Horizontally
When single-server hits limits:

```
                    ┌─────────────┐
                    │   Load      │
                    │  Balancer   │
                    └──────┬──────┘
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
   ┌────▼────┐       ┌─────▼────┐       ┌────▼────┐
   │ World 1 │       │ World 2  │       │ World 3 │
   │ Server  │       │  Server  │       │ Server  │
   └────┬────┘       └────┬─────┘       └────┬────┘
        │                 │                  │
        └─────────────────┼──────────────────┘
                          │
                    ┌─────▼─────┐
                    │   Redis   │ (Pub/Sub for cross-world)
                    │  Cluster  │
                    └─────┬─────┘
                          │
                    ┌─────▼─────┐
                    │ PostgreSQL│ (Persistent data)
                    └───────────┘
```

---

## What You DON'T Need

1. **A custom rendering engine** - WebGL/PixiJS/Three.js are fine for 2D/2.5D
2. **A custom scripting language** - Rust macros or a simple DSL suffice
3. **A complete rewrite** - Iterate on what you have
4. **C++ for performance** - Rust is equally fast, safer, and you already know it

---

## Specific Recommendations for Rustscape

### Immediate Wins
1. **Profile your tick** - Find the actual bottleneck before optimizing
2. **Cache appearance blocks** - Only rebuild when equipment/appearance changes
3. **Implement viewport culling** - Don't send entities outside player's view
4. **Single flush per tick** - Batch all outgoing packets

### Medium-Term
1. **Consider hecs or Bevy ECS** - Clean architecture for game logic
2. **Add Rayon for parallel NPC processing** - Safe parallelism
3. **Implement proper delta updates** - Only send changes

### Long-Term (If Scaling Past 1000+ Players)
1. **Regional servers** - Separate processes per world region
2. **Redis pub/sub** - Cross-region communication
3. **Database sharding** - If PostgreSQL becomes bottleneck

---

## Conclusion

You don't need to build a RuneTek clone. Your Rust + WebSocket architecture is fundamentally sound. Focus on:

1. **Measuring** where time is spent in your tick
2. **Optimizing** the specific bottlenecks you find
3. **Iterating** based on real player load testing

The RSPS community has dealt with these exact problems for 20 years. The standard Java RSPS can handle 2,000+ players on modest hardware with proper optimization. Your Rust server should be able to significantly exceed that.

**Next step**: Add tick profiling to identify your actual bottlenecks, then we can discuss specific optimizations.

---

## Resources

- [RSPS Wiki - Benchmarking](https://rsps.fandom.com/wiki/Benchmarking:_cycle-time)
- [Bevy ECS](https://bevyengine.org/)
- [hecs - Minimal Rust ECS](https://github.com/Ralith/hecs)
- [Rune-Server Forums](https://rune-server.org/) - RSPS development community

This changes everything! For a solo dev with a few friends, you're massively over-engineered right now. Let me give you practical advice for your actual use case.

## Workflow Simplification

**Your current stack for ~5 players:**
- Docker Compose (nginx, postgres, redis, server)
- Kotlin Multiplatform client
- Full RSPS-style architecture

**What you actually need for ~5 players:**
- Single Rust binary
- Static file serving
- SQLite or even JSON files

### Immediate Simplifications

```
What you have now:              What you need:
─────────────────────           ──────────────
nginx reverse proxy      →      Rust serves static files directly
PostgreSQL               →      SQLite (or JSON files for tiny scale)
Redis                    →      In-memory HashMap (it's 5 players)
Docker Compose           →      cargo run
```

## Asset Setup Recommendations

Looking at your structure, you have the right idea with JSON files. Here's how I'd organize for git-friendly solo dev:

```
assets/
├── definitions/          # Game data (git tracked, human-editable)
│   ├── items.json
│   ├── npcs.json
│   ├── objects.json
│   └── shops.json
├── spawns/               # World population
│   ├── npcs/
│   │   ├── lumbridge.json
│   │   └── varrock.json
│   └── objects/
│       └── lumbridge.json
├── maps/
│   └── collision/        # Generated, maybe .gitignore
│       └── *.json
├── cache/                # RS cache files (.gitignore, download separately)
│   └── main_file_cache.*
└── sprites/              # Extracted sprites (git-lfs or .gitignore)
    └── *.png
```

### Git Strategy for Assets

```gitignore
# .gitignore

# Large binary cache (download separately or use git-lfs)
assets/cache/main_file_cache.*
assets/cache/disk.zip

# Generated files (regenerate from cache)
assets/sprites/
assets/maps/collision/

# Keep definitions tracked
!assets/definitions/
!assets/spawns/
```

**For the RS cache files**, either:
1. Git LFS (if you want them in repo)
2. A simple download script that fetches them on first run
3. Document "download cache from X and place in assets/cache/"

## Simplified Server Architecture

For 5 players, here's a minimal setup:

```rust
// main.rs - Everything in one place

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

struct GameState {
    players: HashMap<u32, Player>,
    npcs: Vec<Npc>,
    // That's it. No Redis needed.
}

#[tokio::main]
async fn main() {
    // Load definitions from JSON files at startup
    let items: Vec<ItemDef> = serde_json::from_str(
        &std::fs::read_to_string("assets/definitions/items.json").unwrap()
    ).unwrap();
    
    let npcs: Vec<NpcSpawn> = load_all_spawns("assets/spawns/npcs/");
    
    // SQLite for persistence (or skip entirely for dev)
    let db = rusqlite::Connection::open("world.db").ok();
    
    // Single shared state
    let state = Arc::new(RwLock::new(GameState::default()));
    
    // Serve static files + websocket on same port
    // axum or warp can do both easily
}
```

### File-Based "Database" for Development

For rapid iteration, skip the database entirely during development:

```rust
// Save player data as JSON files
fn save_player(player: &Player) {
    let path = format!("data/players/{}.json", player.username);
    std::fs::write(&path, serde_json::to_string_pretty(player).unwrap()).unwrap();
}

fn load_player(username: &str) -> Option<Player> {
    let path = format!("data/players/{}.json", username);
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
}
```

```
data/
└── players/
    ├── nuniesmith.json    # Human-readable, git-tracked if you want
    └── yourfriend.json
```

**Benefits:**
- `git diff` shows exactly what changed
- Easy to manually edit for testing
- No database migrations
- No Docker needed

## Recommended Dev Workflow

### 1. Hot Reload for Definitions

```rust
// Watch for file changes during development
#[cfg(debug_assertions)]
fn maybe_reload_definitions(state: &mut GameState) {
    if definitions_changed() {
        state.items = load_items();
        state.npcs = load_npc_defs();
        println!("Reloaded definitions!");
    }
}
```

Now you can edit `items.json`, save, and see changes without restarting.

### 2. Simple Dev Scripts

```bash
# run.sh - Development
#!/bin/bash
cd src/server
cargo run

# No Docker, no nginx, no postgres, no redis
```

```bash
# client.sh - Serve client for development  
#!/bin/bash
cd src/clients
./gradlew wasmJsBrowserRun  # or however your Kotlin build works
```

### 3. Single Command Deploy

For playing with friends, you probably just need:

```bash
# deploy.sh
cargo build --release
scp target/release/rustscape yourserver:~/
scp -r assets/ yourserver:~/
ssh yourserver "cd ~ && ./rustscape"
```

Or even simpler—run it on your own machine and use a tunnel:

```bash
# Let friends connect to your local machine
ngrok http 8443
# or
tailscale up  # VPN between friends
```

## What to Remove (For Now)

| Component | Keep/Remove | Why |
|-----------|-------------|-----|
| Docker Compose | Remove | Overkill for 5 players |
| nginx | Remove | Rust can serve files |
| PostgreSQL | Remove | SQLite or JSON files |
| Redis | Remove | In-memory state is fine |
| SSL certs | Simplify | Use Let's Encrypt or skip for dev |

You can always add these back when/if you need scale.

## Suggested Project Structure

```
rustscape/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point, starts server
│   ├── game/             # Game logic
│   │   ├── mod.rs
│   │   ├── player.rs
│   │   ├── npc.rs
│   │   ├── combat.rs
│   │   └── skills.rs
│   ├── net/              # Networking
│   │   ├── mod.rs
│   │   ├── packets.rs
│   │   └── codec.rs
│   └── world/            # World state
│       ├── mod.rs
│       └── region.rs
├── assets/               # Git-tracked game data
│   ├── definitions/
│   └── spawns/
├── cache/                # .gitignored RS cache
├── data/                 # .gitignored runtime data
│   └── players/
├── client/               # Your Kotlin client (or separate repo)
└── run.sh
```

## My Actual Recommendation

1. **Ditch Docker for local dev** — Just `cargo run`
2. **Use JSON files for everything** — Players, definitions, spawns
3. **SQLite only if you want persistence** — And even then, JSON might be easier
4. **Serve client from Rust** — One less moving part
5. **Git track your game data** — Definitions, spawns, even player saves during dev
6. **Add complexity only when needed** — You can always scale up later

The goal is to minimize the time between "I want to change X" and "I can test X in game." Every Docker container, database migration, and build step is friction.

Would you like me to sketch out a simplified single-binary server setup, or help reorganize your asset structure?

Let me create a simplified project structure and example code for you.Done! Here's what I created:

## What's in the tarball

```
rustscape-simple/
├── Cargo.toml              # Minimal deps: axum, tokio, serde, dashmap
├── src/
│   ├── main.rs             # Single entry point, serves HTTP + WebSocket
│   ├── game/mod.rs         # State, tick loop, JSON loading, player saves
│   ├── net/mod.rs          # WebSocket handling, packet definitions
│   └── world/mod.rs        # Region helpers (placeholder)
├── assets/
│   ├── definitions/        # Git-tracked game data
│   │   ├── items.json      # 19 example items
│   │   └── npcs.json       # 11 example NPCs
│   └── spawns/npcs/
│       └── lumbridge.json  # 13 NPC spawns
├── client/dist/
│   └── index.html          # Test client (connect, chat, move)
├── run.sh                  # Just runs cargo run
├── .gitignore              # Ignores data/, cache/, keeps definitions/
└── README.md
```

## Key simplifications

| Before | After |
|--------|-------|
| Docker Compose | `cargo run` |
| PostgreSQL | JSON files in `data/players/` |
| Redis | In-memory `DashMap` |
| nginx | Axum serves static files |
| Binary protocol | JSON over WebSocket (easy to debug) |
| Complex build | Single `cargo run` |

## To use it

```bash
tar -xzf rustscape-simple.tar.gz
cd rustscape-simple
mkdir -p data/players
cargo run
# Open http://localhost:8080
```

## Integrating your existing client

Point your Kotlin client's WebSocket at `ws://localhost:8080/ws` and adapt the packet format, or drop your built client files into `client/dist/`.

The JSON packet format is dead simple to debug—you can see exactly what's happening in browser dev tools.