# Combat System Documentation

## Overview

The Combat System enables players to attack other players and NPCs, with damage calculated based on combat stats (Attack, Strength, Defence). The system includes hit chance calculations, damage formulas, health tracking, death handling, and respawn mechanics.

## Architecture

### Core Components

1. **Combat State** - Tracks who is fighting whom
2. **Health System** - Current HP tracking and max HP calculation
3. **Hit Calculation** - Determines if attacks hit or miss
4. **Damage Formula** - Calculates damage dealt on successful hits
5. **Death & Respawn** - Handles entity death and player respawn
6. **Combat Cooldown** - Prevents attack spam (4 ticks = 2.4 seconds)

---

## Data Structures

### Player Combat Fields

```rust
pub struct Player {
    pub current_hp: u32,                    // Current health points
    pub combat_target: Option<CombatTarget>, // Current attack target
    pub last_combat_tick: u32,               // Last attack tick (for cooldown)
    // ... other fields
}

pub enum CombatTarget {
    Player(u32),  // Attacking player with this ID
    Npc(u32),     // Attacking NPC with this ID
}
```

### NPC Combat Fields

```rust
pub struct Npc {
    pub health: u32,      // Current HP
    pub max_health: u32,  // Maximum HP
    // ... other fields
}
```

---

## Packet Definitions

### Client Packets

```rust
ClientPacket::Attack {
    target_type: String,  // "player" or "npc"
    target_id: u32,       // Entity ID to attack
}
```

### Server Packets

```rust
ServerPacket::CombatHit {
    attacker_id: u32,
    target_id: u32,
    damage: u32,
    target_hp: u32,
    target_max_hp: u32,
}

ServerPacket::Death {
    entity_id: u32,
    killer_id: Option<u32>,
}

ServerPacket::HealthUpdate {
    entity_id: u32,
    current_hp: u32,
    max_hp: u32,
}
```

---

## Combat Mechanics

### 1. Attack Range

**Distance**: Must be within **1 tile** (adjacent) to attack
- Horizontal: 1 tile
- Vertical: 1 tile
- Diagonal: 1 tile
- Same plane (z-level) required

```rust
pub fn in_combat_range(pos1: &Position, pos2: &Position) -> bool {
    if pos1.z != pos2.z {
        return false;
    }
    let dx = (pos1.x - pos2.x).abs();
    let dy = (pos1.y - pos2.y).abs();
    dx <= 1 && dy <= 1
}
```

### 2. Hit Calculation

**Formula**:
```
base_chance = 50%
level_difference = attacker_attack - target_defence
hit_chance = base_chance + (level_difference * 2%)
hit_chance = clamp(hit_chance, 10%, 90%)
```

**Range**: Minimum 10% hit chance, maximum 90% hit chance

**Example**:
- Equal levels (10 vs 10): ~50% hit chance
- High attack (50 vs 10): 50% + (40 * 2%) = 90% hit chance (capped)
- Low attack (10 vs 50): 50% + (-40 * 2%) = 10% hit chance (capped)

```rust
pub fn calculate_hit(attacker_attack_level: u8, target_defence_level: u8) -> bool {
    let level_diff = attacker_attack_level as i32 - target_defence_level as i32;
    let hit_chance = 50.0 + (level_diff as f32 * 2.0);
    let hit_chance = hit_chance.clamp(10.0, 90.0);
    
    let roll = rand::thread_rng().gen_range(0.0..100.0);
    roll < hit_chance
}
```

### 3. Damage Calculation

**Formula**:
```
base_damage = (strength_level + weapon_bonus) / 10
max_hit = max(base_damage, 1)
damage = random(1 to max_hit)
```

**Misses**: Deal 0 damage

**Example**:
- Strength 50, weapon bonus 0: max_hit = 5, damage = 1-5
- Strength 10, weapon bonus 0: max_hit = 1, damage = 1
- Strength 99, weapon bonus 50: max_hit = 14, damage = 1-14

```rust
pub fn calculate_damage(attacker_strength_level: u8, weapon_bonus: u32, hit: bool) -> u32 {
    if !hit {
        return 0;
    }
    let base_damage = (attacker_strength_level as u32 + weapon_bonus) / 10;
    let max_hit = base_damage.max(1);
    rand::thread_rng().gen_range(1..=max_hit)
}
```

### 4. Health Points

**Max HP Formula**:
```
max_hp = hitpoints_level * 10
```

**Examples**:
- Level 1 HP = 10 HP
- Level 10 HP = 100 HP
- Level 99 HP = 990 HP

**New Players**: Start at full HP (level 10 = 100 HP)

```rust
pub fn get_max_hp(hitpoints_level: u8) -> u32 {
    hitpoints_level as u32 * 10
}
```

### 5. Combat Cooldown

**Duration**: 4 game ticks = 2.4 seconds (600ms per tick)

Prevents attack spam by checking:
```rust
if current_tick - last_combat_tick < 4 {
    // Attack on cooldown, ignore request
    return None;
}
```

---

## Combat Flow

### Player vs Player

1. **Client sends Attack packet** with target player ID
2. **Server validates**:
   - Attacker is logged in
   - Target exists
   - Within combat range (1 tile)
   - Not on cooldown
3. **Server calculates hit**:
   - Use attacker's Attack level
   - Use target's Defence level
   - Roll hit/miss
4. **Server calculates damage**:
   - Use attacker's Strength level
   - Use weapon bonus (currently 0)
   - Random damage from 1 to max_hit
5. **Server applies damage**:
   - Subtract damage from target HP
   - Update attacker's combat tick
   - Set attacker's combat target
6. **Server broadcasts CombatHit**:
   - Send to all visible players
   - Includes damage, new HP, max HP
7. **Check for death**:
   - If HP reaches 0:
     - Respawn at Lumbridge (3222, 3218, 0)
     - Restore to full HP
     - Broadcast Death packet
     - Clear combat target

### Player vs NPC

1. **Client sends Attack packet** with target NPC ID
2. **Server validates** (same as PvP)
3. **Server calculates hit**:
   - NPCs use base 10 defence (for now)
4. **Server calculates damage** (same formula)
5. **Server applies damage** to NPC
6. **Server broadcasts CombatHit**
7. **Check for NPC death**:
   - If HP reaches 0:
     - Remove NPC from world
     - Broadcast Death packet
     - NPC will respawn on next tick (future feature)

---

## Attack Handler Implementation

### Validation Checks

```rust
// 1. Player must be logged in
if let Some(pid) = *player_id {
    
    // 2. Get attacker data
    if let Some(mut attacker) = state.players.get_mut(&pid) {
        
        // 3. Check cooldown
        if current_tick - attacker.last_combat_tick < 4 {
            return None;
        }
        
        // 4. Check range
        if !in_combat_range(&attacker_pos, &target_pos) {
            return None;
        }
        
        // 5. Calculate and apply damage
        // ...
    }
}
```

### Damage Application

```rust
// Calculate hit
let hit = calculate_hit(attacker_attack, target_defence);

// Calculate damage
let damage = calculate_damage(attacker_strength, weapon_bonus, hit);

// Apply damage
let new_hp = target_hp.saturating_sub(damage);
target.current_hp = new_hp;

// Update combat state
attacker.last_combat_tick = current_tick;
attacker.combat_target = Some(CombatTarget::Player(target_id));
```

### Broadcasting

```rust
// Broadcast hit to nearby players
send_to_visible_players(
    state,
    &attacker_pos,
    ServerPacket::CombatHit {
        attacker_id: pid,
        target_id,
        damage,
        target_hp: new_hp,
        target_max_hp,
    },
);
```

---

## Client Integration

### Test Client Combat UI

```html
<h3>Combat</h3>
<select id="attackType">
    <option value="player">Player</option>
    <option value="npc">NPC</option>
</select>
<input type="number" id="attackTargetId" placeholder="Target ID">
<button onclick="attackTarget()">Attack</button>
```

### Sending Attack

```javascript
function attackTarget() {
    const targetType = document.getElementById('attackType').value;
    const targetId = parseInt(document.getElementById('attackTargetId').value);
    send({
        type: 'Attack',
        target_type: targetType,
        target_id: targetId
    });
}
```

### Handling Combat Messages

```javascript
ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    if (data.type === 'CombatHit') {
        log(`⚔️ Combat Hit! Attacker: ${data.attacker_id}, ` +
            `Target: ${data.target_id}, Damage: ${data.damage}, ` +
            `HP: ${data.target_hp}/${data.target_max_hp}`, 'info');
    } else if (data.type === 'Death') {
        log(`💀 Death! Entity ${data.entity_id} killed by ${data.killer_id || 'unknown'}`, 'info');
    }
};
```

---

## Testing

### Unit Tests

**11 Combat Tests** (all passing):

1. `test_combat_hit_calculation` - Equal levels = ~50% hit rate
2. `test_combat_hit_advantage` - Higher attack = more hits
3. `test_combat_damage_on_miss` - Misses deal 0 damage
4. `test_combat_damage_on_hit` - Hits deal at least 1 damage
5. `test_combat_damage_scales_with_strength` - Higher strength = more damage
6. `test_max_hp_calculation` - HP formula (level * 10)
7. `test_combat_range_adjacent` - Adjacent tiles in range
8. `test_combat_range_too_far` - 2+ tiles out of range
9. `test_combat_range_different_plane` - Different planes not in range
10. `test_player_starts_with_full_hp` - New players at 100 HP
11. `test_combat_target_none_by_default` - No initial combat target

### Manual Testing

**Setup**:
```bash
cd rustscape/src
cargo run
# Open http://localhost:8080/test-client.html in 2 browser windows
```

**Test PvP**:
1. Window 1: Connect and login as "Player1"
2. Window 2: Connect and login as "Player2"
3. Window 1: Request Players to get Player2's ID
4. Window 1: Set attack type to "player", enter Player2's ID
5. Window 1: Click "Attack"
6. Both windows: See combat hit message
7. Repeat until Player2 dies
8. Window 2: See death message, respawn at Lumbridge

**Test PvE**:
1. Connect and login
2. Click "Request NPCs" to get NPC IDs
3. Set attack type to "npc", enter NPC ID
4. Click "Attack"
5. See combat hit messages
6. Continue until NPC dies
7. See death message, NPC removed

---

## Performance Considerations

### Current Implementation

- **Hit/damage calculations**: O(1) with random number generation
- **Broadcast**: O(N) where N = players in view distance (~15 tiles)
- **Cooldown check**: O(1) tick comparison
- **Target validation**: O(1) DashMap lookup

### Optimization Opportunities

1. **Combat queue system**: Batch attacks processed per tick
2. **Auto-retaliate**: NPCs fight back automatically
3. **Combat styles**: Different attack modes (accurate, aggressive, defensive)
4. **Equipment bonuses**: Cache calculated bonuses

---

## Future Enhancements

### Planned Features

**1. NPC Auto-Retaliate**
- NPCs attack back when attacked
- Aggression radius for hostile NPCs
- NPC AI for targeting

**2. Equipment Bonuses**
- Weapon attack/strength bonuses
- Armor defence bonuses
- Cache bonuses on equip/unequip

**3. Special Attacks**
- Weapon special attacks
- Magic and ranged combat
- Multi-target abilities

**4. Combat XP**
- Grant XP on successful hit
- XP formula: damage * 4 (hitpoints), damage * 4 (attack/strength/defence)
- Level-up system with stat boosts

**5. Loot Drops**
- NPC drop tables
- Item spawn on death
- Loot ownership timers

**6. Prayer System**
- Protection prayers
- Stat-boosting prayers
- Prayer point drain

**7. Combat Animations**
- Attack animations
- Hit splats
- Death animations

---

## Configuration

### Current Values

```rust
// Combat cooldown (ticks)
const COMBAT_COOLDOWN: u32 = 4;  // 2.4 seconds

// Attack range (tiles)
const MELEE_RANGE: i32 = 1;

// Hit chance formula
const BASE_HIT_CHANCE: f32 = 50.0;
const LEVEL_DIFF_MULTIPLIER: f32 = 2.0;
const MIN_HIT_CHANCE: f32 = 10.0;
const MAX_HIT_CHANCE: f32 = 90.0;

// Damage formula
const DAMAGE_DIVISOR: u32 = 10;

// HP formula
const HP_MULTIPLIER: u32 = 10;

// Respawn location
const RESPAWN_X: i32 = 3222;
const RESPAWN_Y: i32 = 3218;
const RESPAWN_Z: i32 = 0;
```

---

## Security Considerations

### Anti-Cheat Measures

1. **Range validation**: Server validates 1-tile range
2. **Cooldown enforcement**: Server-side tick checking
3. **Damage calculation**: All calculations server-side
4. **Entity validation**: Target must exist and be valid
5. **Plane checking**: Same z-level required

### Potential Exploits

1. **Attack spam**: Mitigated by cooldown
2. **Teleport attacks**: Range checked every attack
3. **Damage manipulation**: All RNG server-side
4. **HP editing**: HP stored server-side only

---

## Related Systems

- **Movement System**: Required for combat positioning
- **Visibility System**: Combat events broadcast to nearby players
- **Inventory System**: Future equipment bonuses
- **Skills System**: Attack, Strength, Defence, Hitpoints levels
- **Death System**: Respawn and item dropping

---

## Debugging

### Common Issues

**"Too far to attack"**:
- Check both entities are within 1 tile
- Verify same z-level (plane)

**"Attack on cooldown"**:
- Wait 2.4 seconds between attacks
- Check server tick rate (should be 600ms)

**"No damage dealt"**:
- Attack missed (normal, check hit chance)
- Target may have high defence

**"NPC not dying"**:
- Check NPC max_health value
- Verify damage is being applied

### Debug Logging

```rust
info!("Combat: {} attacked {} for {} damage (hit: {})",
      attacker_name, target_name, damage, hit);

debug!("Player {} attack on cooldown", pid);
debug!("Player {} too far to attack {}", pid, target_id);
```

---

## Changelog

### Version 1.0 (Current)
- Initial combat system implementation
- Player vs Player combat
- Player vs NPC combat
- Hit/miss calculation
- Damage formula
- Death and respawn
- Combat cooldown (2.4 seconds)
- 11 comprehensive unit tests
- Test client integration

---

## See Also

- [PLAYER_SYNC.md](PLAYER_SYNC.md) - Player visibility system
- [EXAMINE_SYSTEM.md](EXAMINE_SYSTEM.md) - Entity examination
- [GROUND_ITEMS.md](GROUND_ITEMS.md) - Item drops (future loot system)
- [FORWARD_PLAN.md](../FORWARD_PLAN.md) - Development roadmap