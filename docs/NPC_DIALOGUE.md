# NPC Dialogue System

**Purpose**: Allow players to interact with NPCs through a branching dialogue system with multiple conversation paths.

---

## Overview

The NPC dialogue system enables players to talk to NPCs and navigate through conversation trees with multiple options. Each dialogue can lead to other dialogues, creating branching conversation paths.

---

## Key Features

1. **Talk to NPCs** - Players can initiate conversation within 3 tiles
2. **Branching Dialogues** - Multiple conversation paths based on player choices
3. **Dialogue Options** - Each dialogue presents multiple response options
4. **Distance Checking** - Must be within 3 tiles and same plane to talk
5. **JSON-Based** - All dialogue content stored in JSON files

---

## Data Structures

### NpcDialogue

```rust
pub struct NpcDialogue {
    pub npc_id: u32,        // NPC definition ID
    pub npc_name: String,   // NPC display name
    pub dialogues: Vec<Dialogue>,  // All dialogue nodes
}
```

### Dialogue

```rust
pub struct Dialogue {
    pub id: u32,              // Unique dialogue ID within this NPC
    pub trigger: String,      // Trigger name (e.g., "greeting")
    pub text: String,         // What the NPC says
    pub options: Vec<DialogueOption>,  // Player response options
}
```

### DialogueOption

```rust
pub struct DialogueOption {
    pub text: String,              // Option text shown to player
    pub next_dialogue: Option<u32>,  // Next dialogue ID (None = end)
}
```

### Storage

Dialogues are stored in `GameState`:

```rust
pub struct GameState {
    // ...
    pub dialogues: DashMap<u32, NpcDialogue>,  // Key: NPC ID
    // ...
}
```

**Loading**: Dialogues load from `assets/dialogue/npcs/*.json` at startup

---

## JSON Format

### File Structure

Each NPC has a separate JSON file in `assets/dialogue/npcs/`:

```json
{
  "npc_id": 0,
  "npc_name": "Hans",
  "dialogues": [
    {
      "id": 0,
      "trigger": "greeting",
      "text": "Hello! Welcome to Lumbridge.",
      "options": [
        {
          "text": "Tell me about Lumbridge.",
          "next_dialogue": 1
        },
        {
          "text": "Goodbye.",
          "next_dialogue": null
        }
      ]
    },
    {
      "id": 1,
      "trigger": "about_lumbridge",
      "text": "Lumbridge is a fine town indeed!",
      "options": [
        {
          "text": "Thanks!",
          "next_dialogue": null
        }
      ]
    }
  ]
}
```

### Dialogue ID Conventions

- **ID 0**: Always the greeting dialogue (first shown when talking to NPC)
- **Other IDs**: Can be any unique number within that NPC's dialogue tree
- **null next_dialogue**: Ends the conversation

### Example Files

**hans.json** - Welcome NPC with town information
**bob.json** - Merchant NPC with shop information (shop not implemented yet)

---

## Packet Types

### Client → Server

```rust
// Initiate conversation with NPC
ClientPacket::TalkToNpc { 
    npc_id: u32  // NPC to talk to
}

// Select a dialogue option
ClientPacket::SelectDialogueOption {
    npc_id: u32,        // NPC being talked to
    dialogue_id: u32,   // Current dialogue ID
    option_index: usize // Which option was selected (0-based)
}
```

### Server → Client

```rust
// Send NPC dialogue to player
ServerPacket::NpcDialogue {
    npc_id: u32,
    npc_name: String,
    dialogue_id: u32,
    text: String,
    options: Vec<DialogueOptionInfo>
}

// DialogueOptionInfo structure
struct DialogueOptionInfo {
    text: String,
    option_index: usize  // Index for selecting this option
}
```

---

## How It Works

### Starting a Conversation

```
1. Player clicks "Talk to Hans" (or sends TalkToNpc packet)
2. Server validates:
   - NPC exists
   - Player within 3 tiles
   - Same plane (z-level)
3. Server looks up dialogue for NPC ID
4. Server sends greeting dialogue (ID 0)
5. Client displays NPC name, text, and options
```

**Code Flow**:
```rust
ClientPacket::TalkToNpc { npc_id } => {
    // Find NPC
    let npc = state.npcs.get(&npc_id)?;
    let npc_pos = npc.position;
    
    // Check distance (within 3 tiles)
    let dx = (player_pos.x - npc_pos.x).abs();
    let dy = (player_pos.y - npc_pos.y).abs();
    
    if dx <= 3 && dy <= 3 && player_pos.z == npc_pos.z {
        // Get dialogue
        let npc_dialogue = state.dialogues.get(&npc_id)?;
        let greeting = npc_dialogue.dialogues.iter().find(|d| d.id == 0)?;
        
        // Send to player
        send(NpcDialogue {
            npc_id,
            npc_name: npc_dialogue.npc_name,
            dialogue_id: 0,
            text: greeting.text,
            options: greeting.options.map_to_info()
        });
    }
}
```

### Selecting an Option

```
1. Player clicks dialogue option
2. Client sends SelectDialogueOption with npc_id, dialogue_id, option_index
3. Server looks up current dialogue
4. Server finds selected option
5. If option has next_dialogue:
   a. Server looks up next dialogue
   b. Server sends next dialogue to player
6. If option.next_dialogue is null:
   a. Conversation ends
   b. Client closes dialogue panel
```

**Code Flow**:
```rust
ClientPacket::SelectDialogueOption { npc_id, dialogue_id, option_index } => {
    // Get NPC dialogue tree
    let npc_dialogue = state.dialogues.get(&npc_id)?;
    let current = npc_dialogue.dialogues.iter().find(|d| d.id == dialogue_id)?;
    
    // Get selected option
    let option = current.options.get(option_index)?;
    
    // Check for next dialogue
    if let Some(next_id) = option.next_dialogue {
        let next = npc_dialogue.dialogues.iter().find(|d| d.id == next_id)?;
        
        send(NpcDialogue {
            npc_id,
            npc_name: npc_dialogue.npc_name,
            dialogue_id: next_id,
            text: next.text,
            options: next.options.map_to_info()
        });
    } else {
        // Conversation ended
    }
}
```

---

## Distance Rules

- **Interaction Range**: 3 tiles in X and Y
- **Plane Check**: Must be on same z-level
- **Distance Formula**: `abs(dx) <= 3 && abs(dy) <= 3 && player.z == npc.z`

This is more lenient than pickup range (1 tile) to make talking easier.

---

## Client Implementation

### JavaScript Example

```javascript
// Talk to NPC
function talkToNpc(npcId) {
    send({ type: "TalkToNpc", npc_id: npcId });
}

// Handle dialogue response
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    if (data.type === "NpcDialogue") {
        showDialogue(data);
    }
};

// Display dialogue
function showDialogue(data) {
    // Show NPC name
    document.getElementById("npcName").textContent = data.npc_name;
    
    // Show dialogue text
    document.getElementById("dialogueText").textContent = data.text;
    
    // Create option buttons
    const optionsDiv = document.getElementById("dialogueOptions");
    optionsDiv.innerHTML = "";
    
    data.options.forEach(option => {
        const button = document.createElement("button");
        button.textContent = option.text;
        button.onclick = () => selectOption(option.option_index);
        optionsDiv.appendChild(button);
    });
}

// Select dialogue option
function selectOption(optionIndex) {
    send({
        type: "SelectDialogueOption",
        npc_id: currentNpcId,
        dialogue_id: currentDialogueId,
        option_index: optionIndex
    });
}
```

---

## Creating New NPC Dialogues

### Step 1: Create JSON File

Create `assets/dialogue/npcs/your_npc.json`:

```json
{
  "npc_id": 5,
  "npc_name": "Your NPC",
  "dialogues": [
    {
      "id": 0,
      "trigger": "greeting",
      "text": "Hello there!",
      "options": [
        {
          "text": "Who are you?",
          "next_dialogue": 1
        },
        {
          "text": "Goodbye.",
          "next_dialogue": null
        }
      ]
    },
    {
      "id": 1,
      "trigger": "who_are_you",
      "text": "I'm a helpful NPC!",
      "options": [
        {
          "text": "That's nice.",
          "next_dialogue": null
        }
      ]
    }
  ]
}
```

### Step 2: Restart Server

Dialogues load at startup:
```bash
cd src && cargo run
```

Should see in logs:
```
Loaded dialogue for NPC: Your NPC
```

### Step 3: Add NPC to World

Update `assets/spawns/npcs/lumbridge.json` to include your NPC at the desired location.

---

## Testing

### Unit Tests

```rust
test_dialogue_loading()   // Checks dialogues load from JSON
test_dialogue_structure() // Validates dialogue tree structure
```

**Run tests**:
```bash
cd src && cargo test
```

### Manual Testing

1. **Start Server**: `cd src && cargo run`
2. **Connect**: Open `http://localhost:8080`
3. **Talk to Hans**: Click "Talk to Hans (NPC 0)"
4. **Expected**: See dialogue panel with Hans's greeting
5. **Select Option**: Click any dialogue option
6. **Expected**: See next dialogue or conversation ends

---

## Dialogue Design Best Practices

### 1. Always Have an Exit Option

Every dialogue should have at least one option with `next_dialogue: null`:

```json
{
  "options": [
    {
      "text": "Tell me more.",
      "next_dialogue": 1
    },
    {
      "text": "Goodbye.",
      "next_dialogue": null
    }
  ]
}
```

### 2. Greeting is ID 0

Always make the first dialogue ID 0 (the greeting):

```json
{
  "dialogues": [
    {
      "id": 0,
      "trigger": "greeting",
      "text": "Hello!"
    }
  ]
}
```

### 3. Keep Text Concise

Dialogue text should be readable at a glance:
- ✅ "Welcome to Lumbridge! I'm Hans, the curator."
- ❌ "Welcome to Lumbridge! I'm Hans, and I've been the curator of this fine town for many years now, and I know every cobblestone and..."

### 4. Use Clear Option Text

Option text should clearly indicate what will happen:
- ✅ "Tell me about Lumbridge."
- ✅ "How can I train my skills?"
- ❌ "Hmm..."

### 5. Logical Flow

Dialogue options should flow naturally:
```
Greeting → About Town → What to Do → Goodbye
         → About NPC  → History     → Goodbye
```

---

## Advanced Features (Future)

### 1. Conditional Dialogues

Based on player stats, quests, items:
```json
{
  "id": 5,
  "trigger": "quest_available",
  "conditions": {
    "min_level": 10,
    "has_item": 100
  },
  "text": "I have a quest for you!"
}
```

### 2. Dialogue Rewards

Give items or XP for completing dialogue:
```json
{
  "id": 10,
  "trigger": "quest_complete",
  "text": "Thanks for helping!",
  "reward": {
    "item_id": 100,
    "amount": 1,
    "xp": 50
  }
}
```

### 3. Shop Integration

Link dialogue to shop system:
```json
{
  "options": [
    {
      "text": "I'd like to buy something.",
      "action": "open_shop",
      "shop_id": 1
    }
  ]
}
```

### 4. Animated NPCs

Trigger NPC animations during dialogue:
```json
{
  "text": "Let me show you!",
  "animation": "wave"
}
```

---

## Performance Notes

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| Load Dialogues | O(N) | N = number of NPC files, done at startup |
| TalkToNpc | O(1) | HashMap lookup by NPC ID |
| SelectOption | O(D) | D = dialogues per NPC (typically < 20) |
| Memory | ~1KB per NPC | Minimal overhead |

**Scalability**: System can handle hundreds of NPCs with thousands of dialogue nodes with negligible performance impact.

---

## Troubleshooting

### Issue: Dialogue not loading

**Check**:
1. JSON file in `assets/dialogue/npcs/`?
2. Valid JSON syntax?
3. Server logs show "Loaded dialogue for NPC: ..."?
4. NPC ID matches file?

**Debug**:
```bash
cd src && cargo run 2>&1 | grep "Loaded dialogue"
```

### Issue: Can't talk to NPC

**Check**:
1. Within 3 tiles?
2. Same plane (z-level)?
3. NPC ID correct?
4. Dialogue file exists for that NPC?

**Debug**:
```javascript
// Console
send({ type: "TalkToNpc", npc_id: 0 });
// Check server logs for distance/validation errors
```

### Issue: Dialogue option doesn't work

**Check**:
1. `next_dialogue` ID exists in dialogues array?
2. Option index correct (0-based)?
3. Server logs for errors?

---

## Related Files

| File | Purpose |
|------|---------|
| `src/src/game/mod.rs` | NpcDialogue, Dialogue, DialogueOption structs |
| `src/src/net/mod.rs` | TalkToNpc and SelectDialogueOption handlers |
| `src/assets/dialogue/npcs/*.json` | Dialogue content files |
| `src/client/dist/index.html` | Client dialogue UI |

---

## Examples

### Simple Linear Conversation

Hans example - each option leads to one response, then ends:

```
Greeting → About Lumbridge → End
         → How Long       → End
         → Goodbye        → End
```

### Branching Conversation

Complex NPC with multiple paths:

```
Greeting → Quest Available? → Accept → Quest Start
                            → Decline → Maybe Later
         → Shop            → Browse  → End
                            → Sell   → End
         → Goodbye         → End
```

---

## Summary

The NPC dialogue system provides:
- ✅ Easy-to-edit JSON dialogue trees
- ✅ Branching conversation paths
- ✅ Distance-based interaction
- ✅ Simple client integration
- ✅ Scalable architecture

**Status**: ✅ Fully implemented and tested  
**Next**: Examine system (Day 13-14)

---

*Last Updated: Day 11-12 - NPC Interaction Implementation*  
*Phase 1, Week 2 in progress*