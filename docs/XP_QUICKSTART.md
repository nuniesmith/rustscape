# XP System Quick Start Guide

**Version**: 1.0  
**Status**: ✅ Live

---

## TL;DR

- **Combat XP**: Damage × 4 per skill
- **Skills**: Attack, Strength, Defence, Hitpoints
- **Level-ups**: Automatic, uses OSRS formula
- **Max Level**: 99
- **Level 2**: 83 XP
- **Level 99**: 13,034,431 XP

---

## How to Gain XP

### Combat (Currently Available)

1. **Attack an NPC or player**
2. **Deal damage**
3. **Receive XP** = damage × 4 per skill

**Example**:
```
Deal 5 damage → 20 XP to Attack
             → 20 XP to Strength
             → 20 XP to Defence
             → 20 XP to Hitpoints
Total: 80 XP across 4 skills
```

---

## Client Messages

### XP Gain Message

```json
{
  "type": "XpGain",
  "skill_name": "Combat",
  "xp_gained": 20,
  "total_xp": 1174
}
```

**Display**: ✨ XP Gain! Combat: +20 XP (Total: 1174)

### Level-Up Message

```json
{
  "type": "LevelUp",
  "skill_name": "Attack",
  "new_level": 11
}
```

**Display**: 🎉 LEVEL UP! Attack is now level 11!

---

## XP Table (Quick Reference)

| Level | XP Required | Typical Hits to Reach |
|-------|-------------|----------------------|
| 1     | 0           | 0                    |
| 2     | 83          | ~21 hits (4 damage)  |
| 5     | 388         | ~97 hits             |
| 10    | 1,154       | ~289 hits            |
| 20    | 4,470       | ~1,118 hits          |
| 30    | 13,363      | ~3,341 hits          |
| 40    | 37,224      | ~9,306 hits          |
| 50    | 101,333     | ~25,333 hits         |
| 60    | 273,742     | ~68,436 hits         |
| 70    | 737,627     | ~184,407 hits        |
| 80    | 1,986,068   | ~496,517 hits        |
| 90    | 5,346,332   | ~1,336,583 hits      |
| 99    | 13,034,431  | ~3,258,608 hits      |

*Assumes average 4 damage per hit*

---

## Testing in Test Client

### Step 1: Login
```
Username: YourName
Password: test
[Login]
```

### Step 2: Find an NPC
```
[Request NPCs]
→ NPCs: [Hans (id: 1), Shopkeeper (id: 2)]
```

### Step 3: Attack
```
Target Type: npc
Target ID: 1
[Attack]
```

### Step 4: Watch for Messages
```
⚔️ Combat Hit! Damage: 3
✨ XP Gain! Combat: +12 XP (Total: 12)
```

### Step 5: Keep Attacking
```
[Attack] ... [Attack] ... [Attack]
```

### Step 6: Level Up!
```
✨ XP Gain! Combat: +28 XP (Total: 84)
🎉 LEVEL UP! Attack is now level 2!
🎉 LEVEL UP! Strength is now level 2!
🎉 LEVEL UP! Defence is now level 2!
🎉 LEVEL UP! Hitpoints is now level 2!
```

---

## Code Examples

### Calculate XP for Damage

```rust
let damage = 5;
let xp = calculate_combat_xp(damage); // Returns 20
```

### Award XP to a Skill

```rust
let mut skill = player.skills.attack;
if let Some(new_level) = add_xp(&mut skill, 20) {
    println!("Leveled up to {}!", new_level);
}
```

### Calculate Level from XP

```rust
let xp = 1154;
let level = level_for_xp(xp); // Returns 10
```

### Calculate XP for Level

```rust
let level = 50;
let xp_needed = xp_for_level(level); // Returns 101333
```

---

## Level Milestones

### Early Levels (Fast Progress)
- **Level 1 → 2**: 83 XP (~21 hits)
- **Level 2 → 3**: 91 XP more (~23 hits)
- **Level 3 → 4**: 102 XP more (~26 hits)

### Mid Levels (Moderate)
- **Level 10 → 20**: 3,316 XP (~829 hits)
- **Level 20 → 30**: 8,893 XP (~2,223 hits)
- **Level 30 → 40**: 23,861 XP (~5,965 hits)

### High Levels (Slow)
- **Level 40 → 50**: 64,109 XP (~16,027 hits)
- **Level 50 → 60**: 172,409 XP (~43,102 hits)
- **Level 60 → 70**: 463,885 XP (~115,971 hits)

### End Game (Very Slow)
- **Level 70 → 80**: 1,248,441 XP (~312,110 hits)
- **Level 80 → 90**: 3,360,264 XP (~840,066 hits)
- **Level 90 → 99**: 7,688,099 XP (~1,922,025 hits)

---

## Tips

### Maximize XP Gains
1. **Higher Strength** = more damage = more XP
2. **Train Attack** to hit more often
3. **Find stronger enemies** (when NPC combat levels matter)

### Level-Up Benefits
- **Attack**: Improves hit chance (+2% per level)
- **Strength**: Increases max damage
- **Defence**: Reduces enemy hit chance (-2% per level)
- **Hitpoints**: Increases max HP (+10 HP per level)

### XP Efficiency
```
Miss (0 damage) → 0 XP
Hit (1 damage)  → 4 XP
Hit (5 damage)  → 20 XP  ← 5x better!
Hit (10 damage) → 40 XP  ← 10x better!
```

**Conclusion**: Higher damage = exponentially faster leveling!

---

## Frequently Asked Questions

### Q: Do I get XP on misses?
**A**: No. Only hits that deal damage award XP.

### Q: Which skills get combat XP?
**A**: Attack, Strength, Defence, and Hitpoints (4 skills total).

### Q: Can I level up multiple levels at once?
**A**: Yes! If you gain enough XP, you can skip levels (e.g., 1 → 5).

### Q: Is XP saved when I logout?
**A**: Yes. All XP and levels are saved in `data/players/`.

### Q: What's the halfway point to 99?
**A**: Level 92 (6,517,253 XP).

### Q: Can I go past level 99?
**A**: Not currently. Level 99 is the maximum.

### Q: Does the attacker or defender get XP?
**A**: Only the attacker (the one dealing damage).

### Q: Do NPCs give different XP?
**A**: No. XP is based purely on damage dealt (damage × 4).

---

## Troubleshooting

### Not Getting XP

**Check**:
1. Are you dealing damage? (Check combat log)
2. Is damage > 0? (Misses don't give XP)
3. Is your WebSocket connected?

**Debug**:
```
Open browser console (F12)
Look for XpGain messages in network tab
```

### Not Leveling Up

**Check**:
1. Do you have enough XP? (Use table above)
2. Check your total XP in XpGain messages
3. Is your level already high?

**Example**:
- You're level 10 (1,154 XP)
- You need 1,210 XP for level 11
- Current XP: 1,200 → Still level 10!
- Need: 10 more XP → 3 hits at 4 damage each

---

## Next Steps

### For Players
1. Test XP system in test client
2. Try leveling up to level 5
3. Check `data/players/YourName.json` to see XP values

### For Developers
1. Read `docs/XP_SYSTEM.md` for full details
2. Run `cargo test xp` to see all tests
3. Implement XP for other skills (woodcutting, mining, etc.)

---

## See Also

- **Full Documentation**: `docs/XP_SYSTEM.md`
- **Combat System**: `docs/COMBAT_SYSTEM.md`
- **Test Coverage**: `cargo test xp`
- **OSRS Wiki**: https://oldschool.runescape.wiki/w/Experience

---

*Quick reference for the Rustscape XP system*  
*Last updated: Current session*