# Rustscape Setup Guide

## ✅ Project Structure Fixed!

The project has been reorganized to work from the root directory. You can now run `cargo` commands from the project root (`rustscape/`) instead of the nested `src/` directory.

## 📁 New Directory Structure

```
rustscape/                    # ← Run cargo commands from here!
├── Cargo.toml               # Rust project configuration
├── Cargo.lock
├── src/                     # Rust source code
│   ├── main.rs             # Server entry point
│   ├── game/               # Game logic module
│   ├── net/                # Networking module
│   └── world/              # World/collision module
├── assets/                  # Game data files
│   ├── definitions/        # Item/NPC definitions (JSON)
│   ├── dialogue/           # NPC dialogue trees
│   └── spawns/             # Spawn locations
├── client/                  # Web client files
│   └── dist/               # Static files served by server
│       ├── index.html      # Landing page (redirects to game)
│       ├── game.html       # Modern game client ⭐
│       └── test-client.html # Debug client
├── data/                    # Runtime data (auto-generated)
│   └── players/            # Player save files (JSON)
├── docs/                    # Documentation
├── scripts/                 # Utility scripts
└── target/                  # Build artifacts (auto-generated)
```

## 🚀 Quick Start

### Run the Server

```bash
# From the project root directory
cd ~/github/rustscape
cargo run
```

You should see:
```
🎮 Rustscape server running at http://localhost:8080
   WebSocket endpoint: ws://localhost:8080/ws
```

### Access the Game

Open your browser to one of these URLs:

1. **Main Game Client** (recommended): http://localhost:8080/game.html
2. **Landing Page** (auto-redirects): http://localhost:8080/
3. **Test Client** (debug): http://localhost:8080/test-client.html

## 🎮 Game Client Features

The new **game.html** client includes:

- ✅ 2D top-down tile-based game view
- ✅ Click-to-move on tiles
- ✅ Click NPCs to attack them
- ✅ Click ground items to pick up
- ✅ Visual HP bars for NPCs
- ✅ Inventory system (28 slots)
- ✅ Skills panel (Attack, Strength, Defence, Hitpoints)
- ✅ Chat box with color-coded messages
- ✅ Level-up notifications
- ✅ Combat feedback (hits, misses, XP gains)
- ✅ Hover tooltips for entities

## 🛠️ Development Commands

All commands should be run from the **project root** (`~/github/rustscape`):

```bash
# Build the project
cargo build

# Run the server
cargo run

# Run tests
cargo test

# Run with release optimizations
cargo run --release

# Check for errors without building
cargo check

# Clean build artifacts
cargo clean

# Apply compiler suggestions (fix warnings)
cargo fix --bin "rustscape" -p rustscape
```

## 📊 What Changed

### Before (Broken Structure)
```
rustscape/
└── src/
    ├── src/         # ← Nested source (WRONG!)
    │   └── main.rs
    ├── assets/      # ← Nested assets
    ├── client/      # ← Nested client
    └── Cargo.toml   # ← Nested Cargo.toml
```

### After (Fixed Structure)
```
rustscape/           # ← Project root
├── Cargo.toml      # ✅ At root
├── src/            # ✅ Source at root level
│   └── main.rs     # ✅ No nesting
├── assets/         # ✅ At root
└── client/         # ✅ At root
```

## 🔧 Path Updates

The server has been configured to serve static files from the correct locations:

- **Client files**: `client/dist/` (index.html, game.html, test-client.html)
- **Game data**: `assets/` (items, NPCs, dialogue, spawns)
- **Player data**: `data/players/` (auto-created)

## ✨ New Game Client

The new **game.html** client provides a much better experience than the old test client:

### Visual Guide
- 🟨 **Yellow square** = You (the player)
- 🟦 **Blue squares** = Other players
- 🟧 **Orange squares** = NPCs (with HP bars)
- 🟡 **Yellow dots** = Ground items
- 🟩 **Green grid** = Walkable tiles

### Controls
- **Left-click tile** → Move to position
- **Left-click NPC** → Attack NPC
- **Left-click ground item** → Pick up item
- **Click inventory slot** → Drop item
- **Type in chat + Enter** → Send message

## 🐛 Troubleshooting

### "No targets specified in manifest"
**Solution**: Make sure you're in the project root (`~/github/rustscape`), not `~/github/rustscape/src`

### "404 Not Found" for client files
**Solution**: Client files must be in `rustscape/client/dist/`, not `rustscape/src/client/dist/`

### Build warnings
The project has some harmless warnings (unused code, unused fields). These can be ignored or fixed with:
```bash
cargo fix --allow-dirty
```

### Server not finding assets
**Solution**: Make sure `assets/` directory is at project root level with:
```bash
ls -la assets/
```

## 📝 Notes

- The server automatically creates the `data/players/` directory on first run
- Player data is saved as JSON files in `data/players/`
- The game runs on a 600ms tick cycle (~1.67 ticks/second)
- View distance is 15 tiles in all directions
- Combat cooldown is 4 ticks (~2.4 seconds)

## 🎯 Next Steps

1. ✅ Run `cargo run` from project root
2. ✅ Open http://localhost:8080/game.html
3. ✅ Login (default: username=Player1, password=password)
4. ✅ Click around to move and explore
5. ✅ Click NPCs to engage in combat
6. ✅ Earn XP and level up your skills!

Enjoy Rustscape! 🦀