# Ground Items System

**Purpose**: Allow players to drop, see, and pick up items on the ground with visibility-based synchronization.

---

## Overview

The ground items system enables players to interact with items on the ground, creating a dynamic world where items can be dropped, seen by nearby players, and picked up. All ground item updates use visibility-based broadcasting to ensure only nearby players receive updates.

---

## Key Features

1. **Drop Items** - Players can drop items from their inventory
2. **Pickup Items** - Players can pick up items within 1 tile
3. **Visibility-Based** - Only see items within 15-tile view distance
4. **Automatic Stacking** - Items stack with existing inventory items
5. **Owner Protection** - Future: Items can be owned for 60 seconds (not yet implemented)

---

## Data Structures

### GroundItem

```rust
pub struct GroundItem {
    pub id: u32,              // Unique ground item ID
    pub item_id: u32,         // Item definition ID
    pub amount: u32,          // Stack size
    pub position: Position,   // World position
    pub owner_id: Option<u32>, // If Some, only owner can pick up (future)
    pub spawn_tick: u32,      // When the item was dropped
}
```

### Storage

Ground items are stored in `GameState`:

```rust
pub struct GameState {
    // ...
    pub ground_items: DashMap<u32, GroundItem>,
    // ...
}
```

**Key**: Unique ground item ID  
**Value**: GroundItem struct  
**Concurrency**: Lock-free reads/writes via DashMap

---

## Packet Types

### Client → Server

```rust
// Request all visible ground items
ClientPacket::RequestGroundItems

// Drop item from inventory
ClientPacket::DropItem { 
    slot: usize  // Inventory slot index (0-27)
}

// Pick up ground item
ClientPacket::PickupItem { 
    ground_item_id: u32  // Unique ground item ID
}
```

### Server → Client

```rust
// New ground item spawned
ServerPacket::GroundItemSpawned {
    item: GroundItemInfo
}

// Ground item removed (picked up)
ServerPacket::GroundItemRemoved {
    ground_item_id: u32
}

// List of visible ground items
ServerPacket::GroundItemList {
    items: Vec<GroundItemInfo>
}

// GroundItemInfo structure
struct GroundItemInfo {
    id: u32,        // Ground item unique ID
    item_id: u32,   // Item definition ID
    amount: u32,    // Stack size
    position: Position
}
```

---

## How It Works

### Dropping an Item

```
1. Client sends DropItem { slot: 0 }
2. Server validates:
   - Slot is valid (0-27)
   - Slot contains an item
3. Server removes item from player inventory
4. Server creates GroundItem at player position
5. Server broadcasts GroundItemSpawned to nearby players
6. Server sends GroundItemSpawned back to dropper
```

**Code Flow**:
```rust
ClientPacket::DropItem { slot } => {
    // Get item from inventory
    if let Some(item) = player.inventory[slot].take() {
        // Create ground item
        let ground_item = GroundItem {
            id: state.next_id(),
            item_id: item.id,
            amount: item.amount,
            position: player.position,
            owner_id: Some(player_id),
            spawn_tick: state.current_tick(),
        };
        
        // Add to world
        state.ground_items.insert(ground_item.id, ground_item);
        
        // Notify nearby players
        send_to_visible_players(&state, &position, GroundItemSpawned { ... });
    }
}
```

### Picking Up an Item

```
1. Client sends PickupItem { ground_item_id: 123 }
2. Server validates:
   - Ground item exists
   - Player is within 1 tile
   - Player is on same plane (z-level)
3. Server removes item from ground_items
4. Server adds to player inventory:
   - If stackable and exists: Add to existing stack
   - Else: Find empty slot
5. If inventory full: Put item back, cancel pickup
6. Server broadcasts GroundItemRemoved to nearby players
7. Server sends GroundItemRemoved back to picker
```

**Code Flow**:
```rust
ClientPacket::PickupItem { ground_item_id } => {
    // Check distance
    let dx = (player_pos.x - item_pos.x).abs();
    let dy = (player_pos.y - item_pos.y).abs();
    
    if dx <= 1 && dy <= 1 && player_pos.z == item_pos.z {
        // Remove from ground
        state.ground_items.remove(&ground_item_id);
        
        // Add to inventory (with stacking)
        for inv_slot in player.inventory.iter_mut() {
            if let Some(existing) = inv_slot {
                if existing.id == item_id {
                    existing.amount += amount;
                    added = true;
                    break;
                }
            }
        }
        
        // Notify nearby players
        send_to_visible_players(&state, &position, GroundItemRemoved { ... });
    }
}
```

### Requesting Ground Items

```
1. Client sends RequestGroundItems
2. Server filters all ground items by visibility
3. Server returns only items within 15 tiles and same plane
```

**Code Flow**:
```rust
ClientPacket::RequestGroundItems => {
    let visible_items: Vec<_> = state.ground_items
        .iter()
        .filter(|item| world::in_view_distance(&viewer_pos, &item.position))
        .map(|item| GroundItemInfo { ... })
        .collect();
    
    return GroundItemList { items: visible_items };
}
```

---

## Visibility Rules

Ground items follow the same visibility rules as players:

- **View Distance**: 15 tiles in X and Y
- **Plane Aware**: Must be on same z-level
- **Square Radius**: `abs(dx) <= 15 && abs(dy) <= 15`

### Helper Function

```rust
pub fn get_visible_ground_items(
    position: &Position,
    all_items: &DashMap<u32, GroundItem>
) -> Vec<RefMulti<u32, GroundItem>>
```

---

## Starter Inventory

New players start with:

```rust
inventory[0] = Some(Item { id: 1, amount: 1 });   // Bronze sword
inventory[1] = Some(Item { id: 2, amount: 10 });  // Logs
inventory[2] = Some(Item { id: 3, amount: 25 });  // Coins
```

This gives players items to test the drop system immediately.

---

## Testing

### Unit Tests

```rust
test_ground_item_visibility()           // Only see items within 15 tiles
test_ground_item_different_planes()     // Can't see items on different z-levels
```

**Run tests**:
```bash
cd src && cargo test
```

### Manual Testing

1. **Start Server**:
   ```bash
   cd src && cargo run
   ```

2. **Connect**: Open `http://localhost:8080`

3. **Drop Item**:
   - Click "Drop Item (slot 0)"
   - Should see: `📦 Item spawned: ID 1 (x1) at (3222, 3218)`

4. **Get Items**:
   - Click "Get Items"
   - Should see: `📦 1 ground item(s) visible`

5. **Pickup Item**:
   - Click "Pickup Item"
   - Should see: `🗑️ Item removed: Ground ID 1`

6. **Multi-Player**:
   - Open second tab, connect as different player
   - Drop item in Tab 1
   - Tab 2 should see: `📦 Item spawned: ...`

---

## Client Usage

### JavaScript Example

```javascript
// Drop item from slot 0
send({ type: "DropItem", slot: 0 });

// Request all visible ground items
send({ type: "RequestGroundItems" });

// Pick up specific ground item
send({ type: "PickupItem", ground_item_id: 123 });

// Handle events
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    if (data.type === "GroundItemSpawned") {
        console.log("Item spawned:", data.item);
        // data.item = { id, item_id, amount, position }
    }
    
    if (data.type === "GroundItemRemoved") {
        console.log("Item removed:", data.ground_item_id);
    }
    
    if (data.type === "GroundItemList") {
        console.log("Items:", data.items);
        // data.items = [{ id, item_id, amount, position }, ...]
    }
};
```

---

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Drop Item | O(N) | Broadcasts to N visible players |
| Pickup Item | O(M) | Checks M inventory slots for stacking |
| Request Items | O(I) | Filters I total ground items |
| Visibility Check | O(1) | Simple distance calculation |

**Scalability Notes**:
- Current: Iterates all ground items to filter by visibility
- Future: Use spatial partitioning (region grid) for O(R) lookup
- Network: Only sends updates to visible players (efficient)

---

## Future Enhancements

### 1. Owner Protection (Planned)

```rust
// Only owner can see/pickup item for first 60 seconds
if ground_item.owner_id == Some(player_id) || 
   current_tick - ground_item.spawn_tick > 100 {
    // Allow pickup
}
```

### 2. Item Despawn (Planned)

```rust
// Remove items after 3 minutes (300 ticks)
if current_tick - ground_item.spawn_tick > 300 {
    state.ground_items.remove(&ground_item.id);
}
```

### 3. Item Persistence (Optional)

```rust
// Save ground items to JSON on server shutdown
// Load ground items on server startup
save_ground_items(&state.ground_items)?;
```

### 4. Item Highlighting (Client-side)

```javascript
// Show different colors for:
// - Your items (green)
// - Other player items (yellow)
// - Available items (white)
```

---

## Common Patterns

### Pattern 1: Drop and Notify

```rust
// 1. Remove from inventory
let item = player.inventory[slot].take();

// 2. Create ground item
let ground_item = GroundItem { ... };
state.ground_items.insert(ground_item.id, ground_item);

// 3. Notify nearby players
send_to_visible_players(&state, &position, GroundItemSpawned { ... });
```

### Pattern 2: Pickup with Stacking

```rust
// 1. Try to stack with existing
for inv_slot in player.inventory.iter_mut() {
    if let Some(existing) = inv_slot {
        if existing.id == item_id {
            existing.amount += amount;
            return; // Stacked!
        }
    }
}

// 2. Find empty slot
for inv_slot in player.inventory.iter_mut() {
    if inv_slot.is_none() {
        *inv_slot = Some(Item { ... });
        return; // Added to empty slot
    }
}

// 3. Inventory full - cancel pickup
```

### Pattern 3: Visibility Filtering

```rust
// Get all ground items visible from position
let visible = state.ground_items
    .iter()
    .filter(|item| world::in_view_distance(&pos, &item.position))
    .collect();
```

---

## Related Files

| File | Purpose |
|------|---------|
| `src/src/game/mod.rs` | GroundItem struct, GameState storage |
| `src/src/net/mod.rs` | Packet handlers for drop/pickup/request |
| `src/src/world/mod.rs` | Visibility helper functions |
| `src/client/dist/index.html` | Test client UI and handlers |
| `docs/PLAYER_SYNC.md` | Related: Player synchronization |

---

## Troubleshooting

### Issue: Can't pick up item

**Check**:
1. Are you within 1 tile? (distance check)
2. Same z-level? (plane check)
3. Is inventory full? (28 slots)
4. Correct ground_item_id?

**Debug**:
```javascript
// Console: Check distance
send({ type: "Move", x: item_x, y: item_y }); // Move to item
send({ type: "PickupItem", ground_item_id: 123 });
```

### Issue: Item not visible

**Check**:
1. Within 15 tiles?
2. Same plane (z-level)?
3. Did you request items? (`RequestGroundItems`)

**Debug**:
```javascript
send({ type: "RequestGroundItems" });
// Check console for GroundItemList
```

### Issue: Item duplicated

This would be a bug. Check server logs for:
- Multiple GroundItemSpawned for same inventory slot
- Item not removed from inventory after drop

---

## Best Practices

1. **Always validate distance** before pickup (server-side)
2. **Use visibility filtering** for all ground item queries
3. **Handle inventory full** gracefully (don't lose items)
4. **Broadcast to nearby players** for all ground item changes
5. **Track ground item IDs** in client for easy pickup

---

## Summary

The ground items system provides a complete item lifecycle:

1. **Drop** - Remove from inventory, spawn on ground
2. **Visibility** - Only nearby players see the item
3. **Pickup** - Add to inventory, remove from ground
4. **Broadcast** - Notify nearby players of changes

**Status**: ✅ Fully implemented and tested  
**Next**: NPC interaction and dialogue system

---

*Last Updated: Day 8-10 - Ground Items Implementation*  
*Phase 1, Week 2 in progress*