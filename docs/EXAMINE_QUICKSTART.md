# Examine System - Quick Start Guide

**5-Minute Guide to Testing the Examine Feature**

---

## 🚀 Quick Start

### 1. Start the Server
```bash
cd rustscape/src
cargo run
```

Wait for: `🎮 Rustscape server running at http://localhost:8080`

### 2. Open Test Client
```
http://localhost:8080/test-client.html
```

### 3. Connect and Login
1. Click **"Connect"**
2. Click **"Login"**

---

## 🎮 Testing Examine Items

### Common Items to Test

| Item ID | Name | Examine Text |
|---------|------|--------------|
| 1 | Coins | "Lovely money!" |
| 1277 | Bronze sword | "A bronze sword." |
| 1511 | Logs | "A log from a tree." |
| 526 | Bones | "Bones from a dead creature." |
| 315 | Shrimps | "Some nicely cooked shrimps." |
| 1265 | Bronze pickaxe | "Used for mining." |
| 1351 | Bronze axe | "A woodcutter's axe." |

### Steps
1. Enter item ID in **"Examine Item"** field (e.g., `1`)
2. Click **"Examine Item"** button
3. Look in message log for: `📖 Examine: Lovely money!`

---

## 🎮 Testing Examine NPCs

### Step 1: Get NPC List
1. Click **"Request NPCs"** button
2. Message log shows nearby NPCs with their IDs

Example output:
```
NpcList: [
  { id: 42, def_id: 3, name: "Hans", position: {...} },
  { id: 43, def_id: 7, name: "Cow", position: {...} }
]
```

### Step 2: Examine NPC
1. Copy an NPC ID from the list (e.g., `42`)
2. Enter ID in **"Examine NPC"** field
3. Click **"Examine NPC"** button
4. Look in log for examine text

### Common NPC Examine Texts

| NPC Name | Def ID | Examine Text |
|----------|--------|--------------|
| Hans | 3 | "Walks around aimlessly." |
| Cow | 7 | "A cow. Mooo!" |
| Chicken | 10 | "Cluck cluck!" |
| Goblin | 198 | "An ugly green creature." |
| Guard | 81 | "He tries to keep the peace." |
| Banker | 494 | "They look after your money." |

---

## 🧪 Testing Edge Cases

### Unknown Item ID
```
Enter: 99999
Result: "📖 Examine: Nothing interesting happens."
```

### Unknown NPC ID
```
Enter: 99999
Result: "📖 Examine: Nothing interesting happens."
```

---

## 📦 Packet Examples

### Examine Item Packet
```json
{
  "type": "ExamineItem",
  "item_id": 1
}
```

### Examine NPC Packet
```json
{
  "type": "ExamineNpc",
  "npc_id": 42
}
```

### Server Response
```json
{
  "type": "ExamineText",
  "text": "Lovely money!"
}
```

---

## 🐛 Troubleshooting

### "Nothing interesting happens."
**Possible causes:**
- Item/NPC ID doesn't exist in definitions
- Typo in ID number
- NPC instance ID vs definition ID mismatch

**Solution:**
- Use **"Request NPCs"** to get valid NPC IDs
- Check `assets/definitions/items.json` for valid item IDs
- Try common IDs: 1 (Coins), 1277 (Bronze sword)

### No response in log
**Possible causes:**
- Not connected to server
- Server not running
- WebSocket error

**Solution:**
- Check status shows "🟢 Connected"
- Verify server is running (cargo run)
- Check browser console for errors

### Wrong examine text
**Possible causes:**
- Using wrong ID number
- Looking at wrong entity type

**Solution:**
- Double-check ID in definitions JSON
- Make sure using correct packet type (Item vs NPC)

---

## 📝 Quick Test Checklist

- [ ] Server running on port 8080
- [ ] Test client loaded
- [ ] Connected and logged in
- [ ] Examine Coins (ID 1) - See "Lovely money!"
- [ ] Examine Bronze sword (ID 1277) - See "A bronze sword."
- [ ] Request NPCs list
- [ ] Examine first NPC from list
- [ ] Try invalid item ID (99999) - See fallback message
- [ ] Try invalid NPC ID (99999) - See fallback message

---

## 🎯 Success Criteria

✅ **Working correctly if:**
- Valid item IDs return correct examine text
- Valid NPC IDs return correct examine text
- Invalid IDs return "Nothing interesting happens."
- All messages appear in log with 📖 emoji
- No errors in browser console
- No server crashes

---

## 📚 Full Documentation

For complete details, see:
- `docs/EXAMINE_SYSTEM.md` - Complete architecture
- `WEEK2_SUMMARY.md` - Week 2 overview
- `TODAY.md` - Latest session notes

---

## 🚀 Next Steps After Testing

Once examine is working:
1. Test with multiple items
2. Test with multiple NPCs
3. Test in multiplayer (2+ browser windows)
4. Verify all starter inventory items have examine text
5. Check all NPCs in spawn files have examine text

---

## 💡 Pro Tips

1. **Use RequestNpcs first** - Always get NPC IDs before examining
2. **Start with common items** - Test Coins (1) first
3. **Check the log** - All examine text shows with 📖 emoji
4. **Try fallback** - Test with ID 99999 to verify error handling
5. **Multiple windows** - Examine works independently per player

---

## 🎮 Ready to Test!

```bash
# Terminal 1: Start server
cd rustscape/src
cargo run

# Browser: Open test client
http://localhost:8080/test-client.html

# Test sequence:
Connect → Login → Examine Item (1) → Success! 🎉
```

**Happy examining! 📖**

---

*Examine System Quick Start - Week 2, Day 13-14*  
*Full docs: docs/EXAMINE_SYSTEM.md*