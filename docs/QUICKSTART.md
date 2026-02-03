# Rustscape Quick Start

**Get up and running in 5 minutes.**

---

## Prerequisites

- **Rust 1.85+**: Install from https://rustup.rs
- **That's it!** No Docker, no databases, no build tools.

---

## Running the Server

```bash
# 1. Navigate to the server directory
cd src

# 2. Run the server
cargo run

# Expected output:
# INFO rustscape > Loaded 150 item definitions
# INFO rustscape > Loaded 25 NPC definitions
# INFO rustscape > Spawned 12 NPCs
# INFO rustscape > 🎮 Rustscape server running at http://localhost:8080
# INFO rustscape >    WebSocket endpoint: ws://localhost:8080/ws
```

## Connecting to the Game

1. Open your browser to: **http://localhost:8080**
2. You'll see the test client interface
3. Enter a username (e.g., "TestPlayer")
4. Click **Connect**
5. You're in!

---

## Test Client Features

The built-in test client (`client/dist/index.html`) provides:

- ✅ WebSocket connection status
- ✅ Login/logout
- ✅ Chat messaging
- ✅ Request player list
- ✅ Request NPC list
- ✅ Random movement testing
- ✅ Ping/pong latency check
- ✅ Packet log viewer

---

## Development Workflow

### Making Changes

```bash
# Edit game logic
vim src/src/game/mod.rs

# Edit item definitions
vim src/assets/definitions/items.json

# Edit NPC spawns
vim src/assets/spawns/npcs/lumbridge.json

# Restart server (Ctrl+C then cargo run)
cargo run
```

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Or trace level for maximum verbosity
RUST_LOG=trace cargo run
```

### Building for Release

```bash
# Build optimized binary
cargo build --release

# Run the release build
./target/release/rustscape

# Binary is at: src/target/release/rustscape (Linux/Mac)
# or: src/target/release/rustscape.exe (Windows)
```

---

## Adding Game Content

### Add a New Item

Edit `src/assets/definitions/items.json`:

```json
{
  "id": 1234,
  "name": "Dragon scimitar",
  "examine": "A powerful curved sword.",
  "stackable": false,
  "tradeable": true,
  "value": 60000
}
```

Restart the server. Done!

### Add a New NPC

Edit `src/assets/definitions/npcs.json`:

```json
{
  "id": 100,
  "name": "King Roald",
  "combat_level": 50,
  "examine": "The King of Varrock."
}
```

### Spawn an NPC in the World

Create or edit `src/assets/spawns/npcs/varrock.json`:

```json
[
  {
    "npc_id": 100,
    "position": { "x": 3210, "y": 3424, "z": 0 },
    "wander_radius": 3
  }
]
```

NPCs will spawn automatically on server start.

---

## Playing with Friends

### Option A: Tailscale (Recommended)

**Setup time**: 5 minutes  
**Pros**: Encrypted, easy, persistent IPs  
**Cons**: Friends need to install Tailscale

```bash
# 1. Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh

# 2. Connect
sudo tailscale up

# 3. Get your IP
tailscale ip -4
# Example output: 100.64.0.5

# 4. Share this with friends
# They connect to: http://100.64.0.5:8080
```

Friends need to:
1. Install Tailscale
2. Join your Tailscale network (you send them an invite link)
3. Open `http://YOUR_TAILSCALE_IP:8080` in their browser

### Option B: ngrok (Quick Testing)

**Setup time**: 2 minutes  
**Pros**: Instant URL, HTTPS included  
**Cons**: URL changes each time, bandwidth limits on free tier

```bash
# 1. Install ngrok
# Download from: https://ngrok.com/download
# Or: sudo snap install ngrok (Linux)

# 2. Start tunnel
ngrok http 8080

# 3. Share the URL
# Example: https://abc123.ngrok.io
```

Friends just open the URL in their browser. No installation needed!

### Option C: Port Forwarding (Traditional)

**Setup time**: 15 minutes  
**Pros**: Direct connection, no third-party service  
**Cons**: Requires router access, exposes network

1. Find your local IP: `ip addr | grep "inet 192"` (Linux/Mac) or `ipconfig` (Windows)
2. Login to your router (usually http://192.168.1.1)
3. Find "Port Forwarding" or "NAT" settings
4. Add rule: External port 8080 → Internal YOUR_LOCAL_IP:8080
5. Find your public IP: `curl ifconfig.me`
6. Share: `http://YOUR_PUBLIC_IP:8080`

**Security note**: This exposes port 8080 to the internet. Use a firewall if needed.

---

## File Locations

| Path | Purpose | Git Tracked? |
|------|---------|--------------|
| `src/src/` | Rust source code | ✅ Yes |
| `src/assets/definitions/` | Item/NPC definitions | ✅ Yes |
| `src/assets/spawns/` | NPC spawn locations | ✅ Yes |
| `src/client/dist/` | Browser client files | ✅ Yes |
| `src/data/players/` | Player save files | ❌ No (.gitignored) |
| `src/target/` | Cargo build artifacts | ❌ No (.gitignored) |

---

## Player Data

Player saves are stored as JSON files in `src/data/players/`.

Example: `src/data/players/TestPlayer.json`

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

### Backing Up Player Data

```bash
# Create a backup
cp -r src/data/players src/data/players.backup.$(date +%Y%m%d)

# Restore from backup
cp -r src/data/players.backup.20240202 src/data/players
```

### Resetting a Player

```bash
# Delete the player's save file
rm src/data/players/TestPlayer.json

# Player will be recreated on next login
```

---

## Troubleshooting

### Server won't compile

```bash
# Update Rust
rustup update

# Clean build artifacts
cargo clean
cargo build
```

### Port 8080 already in use

```bash
# Find what's using the port (Linux/Mac)
lsof -i :8080

# Kill the process
kill -9 <PID>

# Or change the port in src/src/main.rs:
# let addr = SocketAddr::from(([0, 0, 0, 0], 8081));
```

### Client won't connect

1. Check server is running: `curl http://localhost:8080`
2. Check WebSocket: Open browser console (F12) and look for errors
3. Check firewall: `sudo ufw allow 8080` (Linux)
4. Try different browser (Chrome/Firefox recommended)

### Player data not saving

1. Check `src/data/players/` directory exists
2. Check file permissions: `ls -la src/data/players/`
3. Check server logs for errors when saving

### Game is lagging

1. Check CPU usage: Game tick should take <100ms
2. Enable debug logs: `RUST_LOG=debug cargo run`
3. Look for "Tick took XXXms" messages
4. If >600ms, you may have too many players or NPCs

---

## Common Commands

```bash
# Start server (development)
cd src && cargo run

# Start server (release mode, faster)
cd src && cargo build --release && ./target/release/rustscape

# Check if server compiles
cd src && cargo check

# Run tests (when you write them)
cd src && cargo test

# Format code
cd src && cargo fmt

# Lint code
cd src && cargo clippy

# Update dependencies
cd src && cargo update

# Clean build artifacts
cd src && cargo clean
```

---

## Next Steps

1. **Read**: [docs/PROJECT_CONTEXT.md](PROJECT_CONTEXT.md) - Full project overview
2. **Explore**: Browse `src/src/game/mod.rs` - Game logic
3. **Customize**: Edit `src/assets/definitions/` - Add items/NPCs
4. **Build**: Follow the forward plan in PROJECT_CONTEXT.md
5. **Play**: Invite friends and test multiplayer!

---

## Getting Help

- Check server logs (stdout) for error messages
- Look at browser console (F12) for client-side errors
- Review packet logs in the test client
- Check this documentation: `docs/PROJECT_CONTEXT.md`

---

**You're ready to go! Run `cargo run` and start building your game.** 🎮