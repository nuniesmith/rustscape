# Testing Multiplayer Synchronization

**Purpose**: Manual testing guide to verify player synchronization and visibility system works correctly.

## Prerequisites

1. Server is built and running:
```bash
cd rustscape/src
cargo run
```

2. Server should show:
```
🎮 Rustscape server running at http://localhost:8080
   WebSocket endpoint: ws://localhost:8080/ws
Loaded X item definitions
Loaded Y NPC definitions
Spawned Z NPCs
```

## Test 1: Basic Connection

**Setup**: Open one browser tab

1. Navigate to `http://localhost:8080`
2. Click "Connect" (default username: TestPlayer)
3. **Expected**:
   - Status dot turns green
   - "Connected" displayed
   - Log shows: `✅ Logged in! Player ID: X, Position: (3222, 3218)`
   - Log shows: `🗺️ Region: 50,50 (15-tile view radius)`

**Pass Criteria**: Login successful, position displayed

---

## Test 2: Two Players - Enter View

**Setup**: Open two browser tabs side-by-side

### Tab 1:
1. Username: "Player1"
2. Click "Connect"
3. Wait for login

### Tab 2:
1. Username: "Player2"  
2. Click "Connect"
3. Wait for login

**Expected in Tab 1**:
```
✨ Player Player2 (ID: 2) entered view at (3222, 3218)
```

**Expected in Tab 2**:
```
✨ Player Player1 (ID: 1) entered view at (3222, 3218)
```

**Pass Criteria**: Both players see each other's PlayerEnter event

---

## Test 3: Movement Synchronization

**Setup**: Two connected players from Test 2

### Tab 1 (Player1):
1. Click "Move Random" button
2. Observe new position in log

### Tab 2 (Player2):
**Expected**:
```
👤 Player 1 moved to (X, Y)
```

**Pass Criteria**: Player2 sees Player1's movement

---

## Test 4: Player Leaves View

**Setup**: Two connected players, both at spawn (3222, 3218)

### Manual Movement Test:

Since we don't have click-to-walk yet, we'll use the browser console:

**Tab 1** - Open Console (F12), run:
```javascript
// Move far away (30+ tiles)
send({ type: "Move", x: 3260, y: 3260 });
```

**Expected in Tab 2**:
```
👋 Player 1 left view
```

**Tab 1** - Move back close:
```javascript
send({ type: "Move", x: 3225, y: 3220 });
```

**Expected in Tab 2**:
```
✨ Player Player1 (ID: 1) entered view at (3225, 3220)
```

**Pass Criteria**: PlayerLeave and PlayerEnter events fire correctly

---

## Test 5: Multiple Players

**Setup**: Open 3+ browser tabs

1. **Tab 1**: Username "Alice" - Connect
2. **Tab 2**: Username "Bob" - Connect  
3. **Tab 3**: Username "Charlie" - Connect

**Expected**: Each player should see all others in their logs:
```
Tab 1 (Alice):
  ✨ Player Bob entered view
  ✨ Player Charlie entered view

Tab 2 (Bob):
  ✨ Player Alice entered view
  ✨ Player Charlie entered view

Tab 3 (Charlie):
  ✨ Player Alice entered view
  ✨ Player Bob entered view
```

**Pass Criteria**: All players see all other players

---

## Test 6: Request Players List

**Setup**: 2+ connected players

1. Click "Get Players" button in any tab
2. **Expected**:
```
👥 2 player(s) visible (within 15 tiles)
  - PlayerName at (X, Y)
  - PlayerName at (X, Y)
```

**Pass Criteria**: List shows all nearby players

---

## Test 7: Collision Detection

**Setup**: One connected player

1. Click "Test Wall" button
2. **Expected**:
```
🧱 Attempting to walk into wall at (3206, 3228)...
🚶 Moved to (3222, 3218)  ← Should stay at current position (blocked)
```

**Note**: Server log should show:
```
Player X movement blocked: (3222, 3218) -> (3206, 3228)
```

**Pass Criteria**: Movement to blocked tile is rejected

---

## Test 8: View Distance Boundary

**Setup**: Two connected players

**Test exact 15-tile boundary**:

**Tab 1** - Console:
```javascript
// Move exactly 15 tiles away (should still be visible)
send({ type: "Move", x: 3237, y: 3218 });
```

**Expected in Tab 2**: Should still see Player 1 (15 tiles is within range)

**Tab 1** - Console:
```javascript
// Move 16 tiles away (should disappear)
send({ type: "Move", x: 3238, y: 3218 });
```

**Expected in Tab 2**:
```
👋 Player 1 left view
```

**Pass Criteria**: Visibility boundary is exactly 15 tiles

---

## Test 9: Plane Separation

**Setup**: One connected player

**Note**: Since we don't have stairs yet, this tests the anti-cheat:

**Tab 1** - Console:
```javascript
// Try to move to different plane (should be blocked)
send({ type: "Move", x: 3222, y: 3218 });
```

Then check server logs - movement to different planes should be rejected by `can_move_to()`.

**Pass Criteria**: Cannot change planes without proper game mechanic

---

## Test 10: Disconnect Cleanup

**Setup**: Two connected players

1. **Tab 1**: Connect as "Player1"
2. **Tab 2**: Connect as "Player2"
3. Both players see each other
4. **Tab 1**: Close the tab or click "Disconnect"

**Expected in Tab 2**:
```
👋 Player 1 left view
```

**Expected in Server Log**:
```
Player Player1 disconnected
```

**Pass Criteria**: Other players are notified when someone disconnects

---

## Test 11: Chat Broadcast

**Setup**: Two connected players

1. **Tab 1**: Type "Hello!" in chat, click Send
2. **Tab 2**: Should see:
```
💬 Player1: Hello!
```

**Pass Criteria**: Chat works between players

---

## Test 12: NPC Visibility

**Setup**: One connected player

1. Click "Get NPCs" button
2. **Expected**:
```
🎭 X NPC(s) visible (within 15 tiles)
  - Hans at (X, Y)
  - ... other NPCs
```

**Pass Criteria**: Only nearby NPCs are returned

---

## Test 13: Region Tracking

**Setup**: One connected player at spawn (3222, 3218)

1. Note the region in log: `Region: 50,50`
2. Click "Move Random" several times
3. Watch region change as you cross region boundaries (every 64 tiles)

**Example**:
```
Position: (3222, 3218) → Region: 50,50
Position: (3260, 3240) → Region: 50,50 (still in same region)
Position: (3290, 3290) → Region: 51,51 (crossed into new region)
```

**Pass Criteria**: Region ID updates correctly

---

## Test 14: Stress Test - Rapid Movement

**Setup**: Two connected players

**Tab 1** - Console:
```javascript
// Send 10 move commands rapidly
for(let i = 0; i < 10; i++) {
    send({ type: "Move", x: 3220 + i, y: 3218 });
}
```

**Expected**:
- Server should handle all messages
- Tab 2 should receive all movement updates
- No crashes or disconnects

**Pass Criteria**: System handles rapid updates without crashing

---

## Test 15: Persistence Check

**Setup**: One connected player

1. Connect as "TestPlayer"
2. Move to a different location (e.g., 3230, 3225)
3. Disconnect
4. Stop server (Ctrl+C)
5. Check `rustscape/src/data/players/testplayer.json` - should show new position
6. Restart server
7. Reconnect as "TestPlayer"

**Expected**: Player spawns at saved position (3230, 3225), not default (3222, 3218)

**Pass Criteria**: Position persists across restarts

---

## Common Issues & Troubleshooting

### Issue: Players don't see each other

**Check**:
1. Both players connected? (Green status dot)
2. Are they within 15 tiles of each other?
3. Are they on the same plane (z-level)?
4. Check server logs for errors

### Issue: Movement not syncing

**Check**:
1. Server logs show "Player X moved to..."?
2. WebSocket connection still active?
3. Try refreshing both tabs
4. Check browser console for errors

### Issue: Chat not working

**Check**:
1. Broadcast channel working?
2. Check server logs
3. Try reconnecting

### Issue: Server crashes

**Check**:
1. Look at panic message
2. Check for corrupt player JSON files
3. Delete `data/players/*.json` if needed
4. Rebuild: `cargo clean && cargo build`

---

## Performance Checks

### Check 1: Memory Usage

**Terminal**:
```bash
# While server is running with 3-5 players:
ps aux | grep rustscape
```

**Expected**: Memory usage should be reasonable (~50-100 MB for small player count)

### Check 2: Message Count

**Browser Console** - Monitor WebSocket traffic:
```javascript
// Before test
let msgCount = 0;
ws.addEventListener('message', () => msgCount++);

// After moving around for 1 minute
console.log(`Received ${msgCount} messages`);
```

**Expected**: Should only receive messages for visible events (not all server events)

---

## Success Criteria Summary

✅ **All tests passing means**:
1. Players can connect and log in
2. Players see each other when nearby
3. Movement updates are sent to visible players only
4. PlayerEnter/PlayerLeave events work
5. Collision detection prevents invalid moves
6. View distance boundary works (15 tiles)
7. Disconnects are handled cleanly
8. Player data persists
9. No crashes or memory leaks

---

## Next Steps After Testing

If all tests pass:
1. ✅ Mark "Player Synchronization" as complete
2. ✅ Move on to **Phase 1, Week 2**: Ground Items
3. ✅ Document any bugs found
4. ✅ Consider optimization if performance issues found

---

**Last Updated**: Day 5 - Player Synchronization Implementation  
**Tested By**: [Your Name]  
**Date**: [Test Date]  
**Result**: [ ] PASS / [ ] FAIL  
**Notes**: _______________________________________