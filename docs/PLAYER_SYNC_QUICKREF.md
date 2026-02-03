# Player Synchronization - Quick Reference Card

**Last Updated**: Current Session  
**Purpose**: Fast lookup for visibility-based multiplayer system

---

## 🎯 Core Concepts

| Concept | Value | Description |
|---------|-------|-------------|
| **View Distance** | 15 tiles | Square radius in X/Y |
| **Plane Aware** | Yes | Different z-levels can't see each other |
| **Message Delivery** | Targeted | Only send to visible players |
| **Visibility Check** | O(N) | Iterates all players (optimize later) |

---

## 📦 Packet Types

### Server → Client Events

```rust
// Player enters view (login or moves into range)
ServerPacket::PlayerEnter {
    id: u32,
    username: String,
    position: Position,
}

// Player leaves view (logout or moves out of range)
ServerPacket::PlayerLeave {
    id: u32,
}

// Player moved (already visible)
ServerPacket::PlayerMoved {
    id: u32,
    position: Position,
}

// Initial sync on login
ServerPacket::PlayerList {
    players: Vec<PlayerInfo>,
}
```

---

## 🔧 Key Functions

### Check Visibility
```rust
// Returns true if positions within 15 tiles AND same plane
world::in_view_distance(a: &Position, b: &Position) -> bool

// Usage:
if world::in_view_distance(&player_pos, &other_pos) {
    // They can see each other
}
```

### Get Visible Players
```rust
// Returns all players within view distance
world::get_visible_players(
    position: &Position,
    all_players: &DashMap<u32, Player>
) -> Vec<RefMulti<u32, Player>>

// Usage:
let visible = world::get_visible_players(&my_pos, &state.players);
for player in visible.iter() {
    println!("Can see: {}", player.username);
}
```

### Send to Visible Players
```rust
// Send packet to all players who can see a position
send_to_visible_players(
    state: &GameState,
    position: &Position,
    packet: ServerPacket
)

// Usage:
send_to_visible_players(
    &state,
    &player_position,
    ServerPacket::PlayerMoved { id: 123, position }
);
```

---

## 🎮 Common Patterns

### Pattern 1: Player Logs In
```rust
// 1. Store message channel in player
player.sender = Some(tx.clone());

// 2. Notify nearby players
send_to_visible_players(
    state,
    &pos,
    ServerPacket::PlayerEnter { id, username, position: pos }
);

// 3. Send new player their nearby players
let visible = world::get_visible_players(&pos, &state.players);
tx.send(ServerPacket::PlayerList { players: visible });
```

### Pattern 2: Player Moves
```rust
// 1. Calculate visibility before/after
let old_viewers = world::get_visible_players(&old_pos, &state.players);
let new_viewers = world::get_visible_players(&new_pos, &state.players);

// 2. Send movement to all viewers
send_to_visible_players(
    state,
    &new_pos,
    ServerPacket::PlayerMoved { id, position: new_pos }
);

// 3. Send enter to new viewers
for viewer in new_viewers {
    if !old_viewers.contains(viewer.id) {
        viewer.sender.send(ServerPacket::PlayerEnter { ... });
    }
}

// 4. Send leave to old viewers
for viewer in old_viewers {
    if !new_viewers.contains(viewer.id) {
        viewer.sender.send(ServerPacket::PlayerLeave { id });
    }
}
```

### Pattern 3: Player Disconnects
```rust
// 1. Save player data
save_player(&player)?;

// 2. Notify nearby players
send_to_visible_players(
    &state,
    &position,
    ServerPacket::PlayerLeft { id }
);

// 3. Remove from game state
state.players.remove(&id);
```

---

## 🧪 Testing Checklist

```
□ Two players log in → both see PlayerEnter
□ Player moves → other sees PlayerMoved
□ Player moves far away → other sees PlayerLeave
□ Player moves back close → other sees PlayerEnter
□ Player disconnects → other sees PlayerLeft
□ Get Players button → shows only nearby players
□ Movement to wall → rejected by collision
□ 15 tile boundary → correctly enforces visibility
```

---

## 🐛 Common Issues

| Issue | Likely Cause | Fix |
|-------|-------------|-----|
| Players don't see each other | Too far apart (>15 tiles) | Check distance calculation |
| PlayerEnter not firing | Forgot to call `send_to_visible_players` | Add notification |
| Movement not syncing | Player.sender not set | Set on login |
| Crash on disconnect | Using removed player | Clone data before removing |

---

## 📊 Performance Notes

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Login | O(N) | Checks all players for visibility |
| Movement | O(N) | Calculates visibility sets |
| Message Delivery | O(V) | Only sends to V visible players |
| Visibility Check | O(1) | Simple distance calculation |

**Optimization Path**: Use region-based spatial partitioning to reduce O(N) to O(R)

---

## 🔍 Debug Commands

```rust
// Server logs
RUST_LOG=debug cargo run

// Browser console - check who you can see
send({ type: "RequestPlayers" });

// Browser console - move to specific position
send({ type: "Move", x: 3230, y: 3220 });

// Browser console - see raw WebSocket messages
ws.addEventListener('message', e => console.log(JSON.parse(e.data)));
```

---

## 📁 Related Files

| File | Purpose |
|------|---------|
| `src/src/net/mod.rs` | Packet handling, message delivery |
| `src/src/world/mod.rs` | Visibility calculations |
| `src/src/game/mod.rs` | Player struct with sender channel |
| `src/client/dist/index.html` | Test client with event handlers |
| `docs/PLAYER_SYNC.md` | Full documentation |
| `docs/TESTING_MULTIPLAYER.md` | Testing guide |

---

## 💡 Quick Tips

1. **Always check visibility** before sending player-specific events
2. **Clone packets** when sending to multiple players (ServerPacket is Clone)
3. **Handle None sender** gracefully (player might have disconnected)
4. **Test with multiple tabs** - Chrome allows localhost:8080 in multiple tabs
5. **View distance is square**, not circular (15 in X AND 15 in Y)
6. **Plane matters** - z=0 and z=1 can't see each other even if close

---

## 🚀 Next Steps

After mastering player sync:
1. **Ground Items** - Apply same visibility pattern to items
2. **Combat** - Send attack animations only to nearby players
3. **NPC Interaction** - Broadcast NPC state changes
4. **Region Optimization** - Reduce O(N) scans with spatial partitioning

---

**Remember**: Only send what players can see! This is the foundation of scalable multiplayer.

---

*Keep this card handy while developing multiplayer features!*