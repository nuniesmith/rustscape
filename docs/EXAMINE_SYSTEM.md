# Examine System Documentation

## Overview

The Examine system allows players to view descriptive text about items and NPCs in the game world. This is a fundamental quality-of-life feature that provides flavor text and information about game entities.

## Architecture

### Packet Types

#### Client Packets

```rust
ClientPacket::ExamineItem { item_id: u32 }
ClientPacket::ExamineNpc { npc_id: u32 }
```

- **ExamineItem**: Request examine text for an item by its definition ID
- **ExamineNpc**: Request examine text for an NPC by its instance ID

#### Server Packets

```rust
ServerPacket::ExamineText { text: String }
```

- **ExamineText**: Returns the examine description to display to the player

### Data Flow

1. **Client sends examine request** with entity ID
2. **Server looks up definition** from item or NPC definitions
3. **Server extracts examine text** from the definition
4. **Server sends ExamineText** back to the requesting client
5. **Client displays** the examine text to the player

## Implementation Details

### Item Examine Handler

```rust
ClientPacket::ExamineItem { item_id } => {
    // Look up the item definition
    if let Some(item_def) = state.items.iter().find(|i| i.id == item_id) {
        Some(ServerPacket::ExamineText {
            text: item_def.examine.clone(),
        })
    } else {
        Some(ServerPacket::ExamineText {
            text: "Nothing interesting happens.".to_string(),
        })
    }
}
```

- Searches the `state.items` vector for the matching item definition
- Returns the `examine` field from the `ItemDef` struct
- Falls back to "Nothing interesting happens." for unknown items

### NPC Examine Handler

```rust
ClientPacket::ExamineNpc { npc_id } => {
    // Look up the NPC instance to get its def_id
    if let Some(npc) = state.npcs.get(&npc_id) {
        let def_id = npc.def_id;
        
        // Find the NPC definition
        if let Some(npc_def) = state.npc_defs.iter().find(|n| n.id == def_id) {
            Some(ServerPacket::ExamineText {
                text: npc_def.examine.clone(),
            })
        } else {
            Some(ServerPacket::ExamineText {
                text: "Nothing interesting happens.".to_string(),
            })
        }
    } else {
        Some(ServerPacket::ExamineText {
            text: "Nothing interesting happens.".to_string(),
        })
    }
}
```

- First looks up the NPC instance to get its `def_id`
- Then searches `state.npc_defs` for the matching definition
- Returns the `examine` field from the `NpcDef` struct
- Falls back to "Nothing interesting happens." for unknown NPCs

## Data Definitions

### ItemDef

```rust
pub struct ItemDef {
    pub id: u32,
    pub name: String,
    pub examine: String,  // ← Examine text stored here
    pub stackable: bool,
    pub tradeable: bool,
    pub value: u32,
}
```

Example from `assets/definitions/items.json`:

```json
{
  "id": 1,
  "name": "Coins",
  "examine": "Lovely money!",
  "stackable": true,
  "tradeable": true,
  "value": 1
}
```

### NpcDef

```rust
pub struct NpcDef {
    pub id: u32,
    pub name: String,
    pub combat_level: Option<u8>,
    pub examine: String,  // ← Examine text stored here
}
```

Example from `assets/definitions/npcs.json`:

```json
{
  "id": 3,
  "name": "Hans",
  "combat_level": null,
  "examine": "Walks around aimlessly."
}
```

## Client Integration

### Test Client Usage

The test client provides input fields and buttons to examine entities:

```javascript
function examineItem() {
    const itemId = parseInt(document.getElementById('examineItemId').value);
    send({
        type: 'ExamineItem',
        item_id: itemId
    });
}

function examineNpc() {
    const npcId = parseInt(document.getElementById('examineNpcId').value);
    send({
        type: 'ExamineNpc',
        npc_id: npcId
    });
}
```

### Handling Examine Responses

```javascript
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    if (data.type === 'ExamineText') {
        log(`📖 Examine: ${data.text}`, 'info');
    }
};
```

## Testing

### Unit Tests

The examine system includes comprehensive unit tests:

- `test_item_definitions_have_examine_text`: Validates all items have examine text
- `test_npc_definitions_have_examine_text`: Validates all NPCs have examine text
- `test_can_find_item_examine_by_id`: Tests item lookup by ID
- `test_can_find_npc_examine_by_def_id`: Tests NPC lookup by definition ID
- `test_item_examine_text_variety`: Ensures variety in examine descriptions
- `test_npc_with_combat_level`: Tests examining combat NPCs
- `test_npc_without_combat_level`: Tests examining non-combat NPCs

Run tests with:

```bash
cd src
cargo test
```

### Manual Testing

1. **Start the server:**
   ```bash
   cd src
   cargo run
   ```

2. **Open test client:**
   Navigate to `http://localhost:8080/test-client.html`

3. **Connect and login:**
   - Click "Connect"
   - Click "Login"

4. **Test item examine:**
   - Enter item ID (e.g., `1` for Coins)
   - Click "Examine Item"
   - Verify examine text appears in log

5. **Test NPC examine:**
   - Click "Request NPCs" to see NPC IDs
   - Enter an NPC ID from the list
   - Click "Examine NPC"
   - Verify examine text appears in log

## Future Enhancements

### Planned Features

1. **Ground Item Examine**
   - Add `ExamineGroundItem { ground_item_id: u32 }`
   - Would look up the ground item, get its `item_id`, then lookup examine text

2. **Context-Aware Examine**
   - Different examine text based on quest progress
   - Different examine text based on player skills
   - Custom examine text for owned items

3. **Rich Examine Text**
   - Support for formatting (colors, bold, etc.)
   - Multi-line descriptions
   - Dynamic text insertion (e.g., show NPC's current health)

4. **Examine Delay/Cooldown**
   - Optional rate limiting to prevent spam
   - Queue examine requests if player spams

5. **Client UI Improvements**
   - Right-click context menu for examine
   - Hover tooltips showing examine text
   - Examine history panel

### Implementation Notes

**Ground Item Examine Example:**

```rust
ClientPacket::ExamineGroundItem { ground_item_id } => {
    if let Some(ground_item) = state.ground_items.get(&ground_item_id) {
        let item_id = ground_item.item_id;
        drop(ground_item);
        
        if let Some(item_def) = state.items.iter().find(|i| i.id == item_id) {
            Some(ServerPacket::ExamineText {
                text: item_def.examine.clone(),
            })
        } else {
            Some(ServerPacket::ExamineText {
                text: "Nothing interesting happens.".to_string(),
            })
        }
    } else {
        Some(ServerPacket::ExamineText {
            text: "Nothing interesting happens.".to_string(),
        })
    }
}
```

## Performance Considerations

### Current Implementation

- **O(n) lookup**: Linear search through items/NPCs vector
- **Acceptable for small datasets**: Current definitions have ~20 items and ~10 NPCs
- **No caching**: Each examine request does a fresh lookup

### Optimization Opportunities

If the number of items/NPCs grows significantly:

1. **Use HashMap for definitions:**
   ```rust
   pub items: HashMap<u32, ItemDef>,
   pub npc_defs: HashMap<u32, NpcDef>,
   ```
   This would make lookups O(1) instead of O(n).

2. **Cache examine text:**
   ```rust
   pub item_examine_cache: DashMap<u32, String>,
   ```
   Pre-populate on server startup for instant lookups.

3. **Indexed definitions:**
   Keep both Vec (for iteration) and HashMap (for lookups).

## Security Considerations

- **No authentication required**: Players can examine any item/NPC by ID
- **Information disclosure**: Examine reveals entity existence
- **Rate limiting**: Consider adding if examine spam becomes an issue

## Related Systems

- **Item System**: Provides item definitions with examine text
- **NPC System**: Provides NPC definitions with examine text
- **Ground Items**: Future integration for examining items on ground
- **Inventory**: Future integration for examining equipped/carried items

## Changelog

### Version 1.0 (Current)
- Initial implementation
- Support for item and NPC examine
- Test client integration
- Comprehensive unit tests
- Documentation

## See Also

- [GROUND_ITEMS.md](GROUND_ITEMS.md) - Ground items system
- [NPC_DIALOGUE.md](NPC_DIALOGUE.md) - NPC dialogue system
- [PLAYER_SYNC.md](PLAYER_SYNC.md) - Player synchronization
- [FORWARD_PLAN.md](../FORWARD_PLAN.md) - Project roadmap