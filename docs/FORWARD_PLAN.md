# Rustscape Forward Plan

**Last Updated**: February 2024  
**Current Status**: ✅ Infrastructure complete, ready for gameplay development  
**Timeline**: 2-3 months to playable multiplayer game

---

## 🎯 Mission Statement

Build a playable RuneScape-inspired MMORPG that you and 5-10 friends can enjoy together, with working movement, NPCs, items, combat, and skills.

---

## Current State ✅

### What's Working
- ✅ Single-binary Rust server (Axum + Tokio)
- ✅ WebSocket connection and packet handling
- ✅ JSON-based persistence (player saves)
- ✅ Game definitions (items, NPCs) loaded from JSON
- ✅ 600ms game tick loop
- ✅ Test client (HTML/JS) for debugging
- ✅ Chat system (broadcast)
- ✅ Player login/logout
- ✅ Data structures for inventory, skills, equipment

### What's Not Working Yet
- ❌ No visual game client (just test UI)
- ❌ Can't interact with NPCs
- ❌ Can't pick up/drop items
- ❌ No combat system
- ❌ Skills don't train

---

## The Plan: 3 Phases

```
Phase 1 (Week 1-2): Core Gameplay Loop
  └─> Movement, collision, items, NPC interaction

Phase 2 (Week 3-6): Content Systems
  └─> Combat, skills, progression

Phase 3 (Week 7-12): Multiplayer Polish
  └─> Client UI, networking, deployment
```

---

## 📅 Phase 1: Core Gameplay Loop (Weeks 1-2)

**Goal**: Walk around, talk to NPCs, pick up items.

### Week 1: Movement & World

#### Day 1-2: Collision Detection ✅ COMPLETED
**File**: `src/src/world/mod.rs`

```rust
// Tasks:
- [✅] Create collision map data structure
- [✅] Load collision from JSON (or hardcode Lumbridge area)
- [✅] Implement can_move_to(from, to) -> bool
- [✅] Test: Can't walk through walls/objects
```

**Deliverable**: Player can walk around but blocked by walls.
**Status**: ✅ Implemented with hardcoded Lumbridge collision tiles

#### Day 3-4: Region System ✅ COMPLETED
**File**: `src/src/world/mod.rs`

```rust
// Tasks:
- [✅] Implement get_visible_players(player) -> Vec<Player>
- [✅] Implement get_visible_npcs(player) -> Vec<Npc>
- [✅] Use region ID for spatial queries
- [✅] Test: Only see players/NPCs in nearby regions
```

**Deliverable**: Optimized visibility checks.
**Status**: ✅ 15-tile view distance with plane awareness

#### Day 5-7: Player Synchronization ✅ COMPLETED
**Files**: `src/src/net/mod.rs`, `src/src/game/mod.rs`

```rust
// Tasks:
- [✅] When player moves, broadcast to nearby players
- [✅] Handle PlayerMoved packet in client
- [✅] Update test client to show other players
- [✅] Test: Two browser tabs, see each other move
- [✅] Add PlayerEnter/PlayerLeave events
- [✅] Visibility-based message delivery
```

**Deliverable**: Multiplayer movement works.
**Status**: ✅ Full visibility-based synchronization with enter/leave events
**Docs**: See `docs/PLAYER_SYNC.md` and `docs/DAY5_PLAYER_SYNC.md`

---

### Week 2: Interaction & Items

#### Day 8-10: Ground Items ✅ COMPLETED
**File**: `src/src/game/mod.rs`

```rust
// Tasks:
- [✅] Add ground_items: DashMap<u32, GroundItem>
- [✅] Implement DropItem packet handler
- [✅] Implement PickupItem packet handler
- [✅] Implement RequestGroundItems with visibility
- [✅] Test: Drop item, see it on ground, pick it back up
```

**Deliverable**: Working item system.
**Status**: ✅ Complete with visibility-based updates and stacking
**Docs**: See `docs/GROUND_ITEMS.md`

#### Day 11-12: NPC Interaction
**Files**: `src/src/game/mod.rs`, `src/src/net/mod.rs`

```rust
// Tasks:
- [ ] Add ClientPacket::TalkToNpc { npc_id }
- [ ] Create dialogue system (JSON-based)
- [ ] Load dialogue from assets/dialogue/npcs.json
- [ ] Send dialogue to client
- [ ] Test: Click NPC, see dialogue
```

**Deliverable**: Can talk to NPCs.

#### Day 13-14: Examine System
**Files**: `src/src/net/mod.rs`

```rust
// Tasks:
- [ ] Add ClientPacket::ExamineItem { item_id }
- [ ] Add ClientPacket::ExamineNpc { npc_id }
- [ ] Send examine text from definitions
- [ ] Test: Right-click examine shows description
```

**Deliverable**: Examine items and NPCs.

---

## 📅 Phase 2: Content Systems (Weeks 3-6)

**Goal**: Combat, skills, and progression.

### Week 3-4: Combat System

#### Combat Core
**File**: `src/src/game/combat.rs` (new module)

```rust
// Tasks:
- [ ] Create combat.rs module
- [ ] Implement attack_npc(player, npc) -> CombatResult
- [ ] Calculate hit chance based on levels/equipment
- [ ] Calculate damage based on weapon/strength
- [ ] Handle NPC retaliation
- [ ] Death handling (player and NPC)
- [ ] Respawn system
```

#### Combat Integration
```rust
// Tasks:
- [ ] Add ClientPacket::AttackNpc { npc_id }
- [ ] Process combat in game tick
- [ ] Send combat updates to client
- [ ] XP gain for combat
- [ ] Test: Kill a chicken, gain XP
```

**Deliverable**: Working combat system.

---

### Week 5-6: Skills System

#### Gathering Skills
**File**: `src/src/game/skills.rs` (new module)

```rust
// Woodcutting:
- [ ] Define tree objects in assets/spawns/objects.json
- [ ] Implement chop_tree(player, tree_id)
- [ ] Random success based on level
- [ ] Grant logs + XP
- [ ] Tree respawn timer

// Fishing:
- [ ] Define fishing spots
- [ ] Implement fish_spot(player, spot_id)
- [ ] Random catch based on level
- [ ] Grant fish + XP

// Mining:
- [ ] Define rocks
- [ ] Implement mine_rock(player, rock_id)
- [ ] Random ore based on level
- [ ] Grant ore + XP
```

#### Skill Progression
```rust
// Tasks:
- [ ] Implement grant_xp(player, skill, amount)
- [ ] Calculate level from XP (RuneScape formula)
- [ ] Send level-up messages
- [ ] Save skill data to player JSON
```

**Deliverable**: 3 gathering skills working.

---

## 📅 Phase 3: Client & Deployment (Weeks 7-12)

**Goal**: Proper game client and multiplayer deployment.

### Week 7-9: Game Client

#### Option A: Canvas/PixiJS Client (Recommended)
```javascript
// Tasks:
- [ ] Set up PixiJS in client/dist/
- [ ] Load sprite data (or use placeholder graphics)
- [ ] Render game world (tile-based)
- [ ] Render players with movement animation
- [ ] Render NPCs
- [ ] Click-to-walk movement
- [ ] Inventory UI
- [ ] Chat box
- [ ] Skills panel
```

#### Option B: Text-Based Client (Faster)
```javascript
// Tasks:
- [ ] ASCII map rendering
- [ ] Command-based movement (WASD)
- [ ] Text inventory
- [ ] MUD-style interface
```

**Deliverable**: Playable game client.

---

### Week 10-11: Polish & Testing

#### Gameplay Polish
```rust
// Tasks:
- [ ] Add sound effects (optional)
- [ ] Add music (optional)
- [ ] Smooth movement interpolation
- [ ] Better chat UI
- [ ] Minimap
- [ ] Loading screens
```

#### Bug Fixing
```rust
// Known issues to fix:
- [ ] Add input validation
- [ ] Handle disconnections gracefully
- [ ] Prevent duplication exploits
- [ ] Add rate limiting
- [ ] Fix memory leaks (if any)
```

#### Testing Checklist
```
- [ ] Solo play for 1 hour (no crashes)
- [ ] 2 players simultaneously
- [ ] 5 players simultaneously
- [ ] Player data persists across restarts
- [ ] No item duplication bugs
- [ ] Combat works correctly
- [ ] Skills train correctly
```

---

### Week 12: Deployment

#### Choose Networking
- [ ] Set up Tailscale for you and friends
- [ ] Test connection from external network
- [ ] Document connection process

#### Create Distribution Package
```bash
# Package script:
- [ ] Build release binary
- [ ] Include assets
- [ ] Include client files
- [ ] Create run script
- [ ] Write README for players
```

#### Player Documentation
- [ ] Write HOW_TO_CONNECT.md
- [ ] Create control guide
- [ ] List features
- [ ] Known issues
- [ ] Screenshots/video

**Deliverable**: Friends can download and play.

---

## 🔧 Development Guidelines

### Daily Workflow
```bash
# 1. Pick one task from current week
# 2. Create a branch (optional)
git checkout -b feature/collision-detection

# 3. Implement the feature
vim src/src/world/mod.rs

# 4. Test immediately
cd src && cargo run
# Open http://localhost:8080 and test

# 5. Commit when working
git add .
git commit -m "Add collision detection"

# 6. Move to next task
```

### Testing Strategy
- ✅ Test each feature immediately after implementing
- ✅ Keep test client open while developing
- ✅ Test with 2+ browser tabs for multiplayer
- ✅ Save player data frequently
- ✅ Restart server often to test persistence

### Code Quality
- ⚠️ Don't worry about perfect code yet
- ✅ Focus on working features first
- ✅ Refactor when things get messy
- ✅ Comment complex logic
- ✅ Use descriptive variable names

---

## 📊 Progress Tracking

### Phase 1 Checklist (Core Loop)
- [✅] Collision detection working
- [✅] Region-based visibility
- [✅] Player movement syncs between clients
- [✅] Ground items spawn/pickup/drop
- [ ] NPC dialogue system
- [ ] Examine items/NPCs

### Phase 2 Checklist (Content)
- [ ] Combat system (attack NPCs)
- [ ] Player vs NPC combat
- [ ] Death and respawn
- [ ] Woodcutting skill
- [ ] Fishing skill
- [ ] Mining skill
- [ ] XP and level-up system

### Phase 3 Checklist (Polish)
- [ ] Proper game client (not test UI)
- [ ] Visual rendering (sprites or ASCII)
- [ ] Inventory UI
- [ ] Chat UI
- [ ] Multiplayer tested with friends
- [ ] Deployment package created

---

## 🎯 Definition of "Done"

The game is ready when:

1. ✅ You can `cargo run` and it starts without errors
2. ✅ Friends can connect via Tailscale/ngrok
3. ✅ **Players can walk around without clipping through walls**
4. ✅ **Players can see each other move in real-time**
5. ✅ Players can talk to NPCs and get responses
6. ✅ Players can pick up and drop items
7. ✅ Players can attack and kill NPCs
8. ✅ Players can train at least one skill (woodcutting/fishing/mining)
9. ✅ Player data saves and loads correctly
10. ✅ You can play for 30 minutes without crashes

**Everything else is bonus content!**

---

## 🚨 Scope Creep Warning

### DO NOT Add These (Yet)
- ❌ Quests (add after core loop works)
- ❌ Trading (add after multiplayer is stable)
- ❌ Clans/friends lists (add in version 2)
- ❌ Banks (add after items work)
- ❌ Player vs Player combat (add way later)
- ❌ Grand Exchange (add never, too complex)

### Focus On
- ✅ Walking around
- ✅ Talking to NPCs
- ✅ Picking up items
- ✅ Killing NPCs
- ✅ Training skills
- ✅ Playing with friends

---

## 📈 Success Metrics

### Week 2 Goal
- Can walk around Lumbridge
- Can pick up items
- Can talk to Hans

### Week 6 Goal
- Can kill chickens
- Can chop trees
- Can gain levels

### Week 12 Goal
- 5 friends playing together
- No game-breaking bugs
- Having fun

---

## 🆘 When You Get Stuck

### Stuck on Implementation?
1. Read `docs/PROJECT_CONTEXT.md` for architecture
2. Look at existing code in `src/src/game/mod.rs`
3. Simplify the feature (make it work first, optimize later)
4. Use placeholder data if needed

### Stuck on Bugs?
1. Add `RUST_LOG=debug` logging
2. Check browser console (F12)
3. Add `println!()` statements
4. Test with minimal scenario
5. Check player JSON files for corruption

### Stuck on Design Decisions?
1. Choose the simplest option
2. Make it work first
3. You can always refactor later
4. Don't block progress on perfection

---

## 📚 Reference Documentation

- **Architecture**: `docs/PROJECT_CONTEXT.md`
- **Quick Start**: `docs/QUICKSTART.md`
- **Optimization**: `docs/todo.md`
- **What Changed**: `docs/CLEANUP_SUMMARY.md`

---

## 🎮 Let's Build This!

**Current Phase**: Phase 1, Week 2 - Interaction & Items
**Current Task**: Day 11-12 - NPC Interaction
**Previous Completed**: ✅ Collision Detection, ✅ Region System, ✅ Player Synchronization, ✅ Ground Items

```bash
cd src
vim src/world/mod.rs
# Implement can_move_to() function
cargo run
# Test it works!
```

**You got this.** 🚀

---

*Updated: February 2024*  
*Next Review: After Phase 1 completion*