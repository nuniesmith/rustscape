# 🎮 Rustscape - Start Here

**Welcome to your RuneScape-inspired MMORPG project!**

This document is your 5-minute orientation to get you up and running.

---

## ⚡ Quick Start (30 seconds)

```bash
cd src
cargo run
# Open http://localhost:8080
```

**That's it.** No Docker, no databases, no complex setup.

---

## 📋 Project Status

### ✅ What's Working Right Now
- Single-binary Rust server (Axum + WebSocket)
- Player login/logout with JSON persistence
- Chat system (broadcast messages)
- Game tick loop (600ms cycle)
- Item & NPC definitions loaded from JSON
- Test client for debugging at http://localhost:8080
- Tailscale connected (friends can join at http://100.69.78.116:8080)

### 🚧 What Needs Work
- No visual client (just test UI)
- Movement doesn't check collision
- Can't interact with NPCs yet
- Can't pick up/drop items
- No combat system
- Skills don't train yet

---

## 📚 Documentation Map

**Read these in order:**

1. **START_HERE.md** ← You are here
2. **[QUICKSTART.md](QUICKSTART.md)** - How to run, build, deploy
3. **[PROJECT_CONTEXT.md](PROJECT_CONTEXT.md)** - Full architecture & technical details
4. **[FORWARD_PLAN.md](FORWARD_PLAN.md)** - 12-week roadmap to playable game

**Optional:**
- **[CLEANUP_SUMMARY.md](CLEANUP_SUMMARY.md)** - What was removed from old architecture
- **[todo.md](todo.md)** - Performance optimization notes
- **[ARCHIVE_old_deployment_plan.md](ARCHIVE_old_deployment_plan.md)** - Outdated, for reference only

---

## 🗂️ Project Structure

```
rustscape/
├── src/                    # The entire server lives here
│   ├── src/               # Rust source code
│   │   ├── main.rs        # Entry point (90 lines)
│   │   ├── game/mod.rs    # Game logic, tick loop
│   │   ├── net/mod.rs     # WebSocket & packets
│   │   └── world/mod.rs   # Collision & regions
│   │
│   ├── assets/            # Game data (JSON)
│   │   ├── definitions/   # items.json, npcs.json
│   │   └── spawns/        # NPC spawn locations
│   │
│   ├── data/              # Runtime data (.gitignored)
│   │   └── players/       # Player save files (JSON)
│   │
│   ├── client/dist/       # Browser client
│   │   └── index.html     # Test client UI
│   │
│   └── Cargo.toml         # Dependencies (minimal)
│
└── docs/                  # All documentation
```

---

## 🎯 What This Project Is

A **2009-era RuneScape-inspired MMORPG** being rebuilt in Rust as a simple, playable game for small groups (5-10 players).

### Core Philosophy
- **Simplicity over scalability**: JSON files instead of PostgreSQL
- **Single binary**: No Docker, nginx, Redis, or databases
- **Rapid iteration**: Edit JSON → restart → test immediately
- **Fun first**: Working gameplay beats perfect architecture

### Technical Stack
- **Server**: Rust, Tokio (async), Axum (web framework)
- **Client**: HTML/JavaScript (test UI) → Will upgrade to proper game client
- **Protocol**: WebSocket with JSON packets
- **Storage**: JSON files on disk

---

## 🚀 Next Steps (Choose Your Path)

### Path A: Just Run It (5 minutes)
```bash
cd src
cargo run
# Visit http://localhost:8080
# Click "Connect" and explore the test client
```

### Path B: Understand It (30 minutes)
1. Read [PROJECT_CONTEXT.md](PROJECT_CONTEXT.md)
2. Browse `src/src/main.rs` and `src/src/game/mod.rs`
3. Look at `src/assets/definitions/items.json`
4. Run the server and test the WebSocket connection

### Path C: Start Building (Today!)
1. Read [FORWARD_PLAN.md](FORWARD_PLAN.md)
2. Start Phase 1, Week 1: Collision Detection
3. Edit `src/src/world/mod.rs`
4. Implement `can_move_to()` function
5. Test it works!

---

## 🎮 The Goal (12 Weeks)

Build a game where you and friends can:
- ✅ Walk around a map (with collision)
- ✅ See each other in real-time
- ✅ Talk to NPCs
- ✅ Pick up and drop items
- ✅ Fight NPCs and gain XP
- ✅ Train skills (woodcutting, fishing, mining)
- ✅ Level up and progress

**Everything else is bonus.**

---

## 🔧 Essential Commands

```bash
# Run the server
cd src && cargo run

# Build release version
cd src && cargo build --release

# Debug logging
cd src && RUST_LOG=debug cargo run

# Check if code compiles
cd src && cargo check

# Run health check
./scripts/diagnose.sh
```

---

## 🌐 Playing with Friends

You already have Tailscale set up! Friends can connect to:

**http://100.69.78.116:8080**

They just need:
1. Tailscale installed
2. Joined to your Tailscale network
3. A browser

That's it!

---

## 📊 Current Stats

| Metric | Value |
|--------|-------|
| Lines of Rust code | ~1000 |
| Item definitions | 19 |
| NPC definitions | 11 |
| NPCs spawned | 12 |
| Saved players | 1 |
| Server binary size | 2.9 MB |
| Dependencies | 9 crates |
| Build time (release) | ~45 seconds |

---

## 🐛 Known Issues

- Port 8080 check in diagnose.sh has inverted logic (cosmetic)
- No collision detection (can walk through walls)
- Player movement doesn't sync to other clients yet
- No authentication (password field ignored)
- NPCs don't move yet

**None of these block development!**

---

## ❓ Common Questions

### "Do I need Docker?"
No. That was the old architecture. It's been removed.

### "Do I need PostgreSQL or Redis?"
No. Players save as JSON files in `src/data/players/`.

### "Where's the Kotlin client?"
That was the old plan. You're using a simple HTML/JS test client now. You'll build a proper client in Phase 3.

### "Can I add items/NPCs?"
Yes! Edit `src/assets/definitions/*.json` and restart the server.

### "How do I share with friends?"
Your Tailscale IP is `100.69.78.116`. They connect to `http://100.69.78.116:8080`.

### "What if I break something?"
Player data is backed up automatically. Worst case: `git reset --hard` and restart.

---

## 🎓 Learning Resources

- **Rust Book**: https://doc.rust-lang.org/book/
- **Tokio Docs**: https://tokio.rs/
- **Axum Guide**: https://docs.rs/axum/latest/axum/
- **OSRS Wiki**: https://oldschool.runescape.wiki/ (game mechanics)

---

## 🎯 Your Immediate Task

**Read [FORWARD_PLAN.md](FORWARD_PLAN.md) and start Phase 1, Day 1: Collision Detection.**

Open `src/src/world/mod.rs` and implement the `can_move_to()` function.

You have everything you need. The infrastructure is done. Now it's time to build the game.

---

## 💭 Remember

- **Ship features, not perfection**
- **Test immediately after coding**
- **Commit often**
- **Focus on gameplay over optimization**
- **Have fun building this!**

---

**Ready? Let's build an MMORPG.** 🚀

```bash
cd src
cargo run
# Open http://localhost:8080
# Let's go!
```

---

*Last Updated: February 2024*  
*Questions? Check PROJECT_CONTEXT.md or FORWARD_PLAN.md*