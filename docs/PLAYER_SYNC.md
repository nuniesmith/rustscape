# Player Synchronization & Visibility System

This document describes how Rustscape implements efficient multiplayer synchronization using region-based visibility.

## Overview

Rather than broadcasting all player movements to all connected players, Rustscape uses a **visibility-based synchronization** system that only sends updates to players who can actually see each other in the game world.

## Key Concepts

### View Distance

- **Range**: 15 tiles in both X and Y directions
- **Shape**: Square/diamond around the player
- **Plane-aware**: Players on different planes (z-levels) cannot see each other, even if close in x/y coordinates

### Visibility Events

The system uses three core event types:

1. **`PlayerEnter`** - Sent when a player comes into view
2. **`PlayerLeave`** - Sent when a player leaves view  
3. **`PlayerMoved`** - Sent when a visible player moves

## How It Works

### Login Flow

When a player logs in:

```
1. Player sends Login packet
2. Server creates/loads player data
3. Server stores a message channel (sender) in the Player struct
4. Server notifies nearby players with PlayerEnter event
5. Server sends the new player:
   - LoginSuccess (with their own position/stats)
   - PlayerList (all visible players within 15 tiles)
   - NpcList (all visible NPCs within 15 tiles)
```

### Movement Flow

When a player moves:

```
1. Player sends Move packet with (x, y) coordinates
2. Server validates:
   - Collision check (blocked tiles)
   - Distance check (max 2 tiles per move - anti-cheat)
   - Plane check (can't change z without stairs/ladder)
3. If valid:
   a. Calculate old viewer set (players who could see old position)
   b. Calculate new viewer set (players who can see new position)
   c. Send PlayerMoved to all players in either set
   d. Send PlayerEnter to players in new set but not old set
   e. Send PlayerLeave to players in old set but not new set
4. If invalid:
   - Send current position back to client (rejection)
```

### Disconnect Flow

When a player disconnects:

```
1. Connection closes
2. Server removes player from GameState.players
3. Server saves player data to JSON
4. Server sends PlayerLeave to all nearby players
```

## Architecture

### Message Delivery

Each player connection has:
- **WebSocket sender/receiver** (for client communication)
- **mpsc::UnboundedChannel** (for server-to-player messages)
- **broadcast::channel** (for global messages like chat)

The connection handler uses `tokio::select!` to multiplex:
- Incoming WebSocket messages from client
- Outgoing targeted messages from server
- Global broadcast messages

### Visibility Calculation

```rust
pub fn get_visible_players(
    position: &Position,
    all_players: &DashMap<u32, Player>
) -> Vec<RefMulti<u32, Player>>
```

This function:
1. Iterates through all online players (using DashMap for lock-free reads)
2. Filters by `in_view_distance()` check
3. Returns references to visible players

The `in_view_distance()` check:
```rust
fn in_view_distance(a: &Position, b: &Position) -> bool {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    dx <= 15 && dy <= 15 && a.z == b.z
}
```

### Targeted Broadcasting

```rust
fn send_to_visible_players(
    state: &GameState,
    position: &Position,
    packet: ServerPacket
)
```

This helper function:
1. Gets all players who can see the given position
2. Iterates through them
3. Sends the packet to each player's dedicated channel

## Data Structures

### Player Struct (Simplified)

```rust
pub struct Player {
    pub id: u32,
    pub username: String,
    pub position: Position,
    pub skills: Skills,
    pub inventory: Vec<Option<Item>>,
    pub equipment: Equipment,
    
    #[serde(skip)]
    pub sender: Option<mpsc::UnboundedSender<ServerPacket>>,
}
```

The `sender` field is:
- Set when the player logs in
- Used to send targeted messages directly to that player
- Skipped during JSON serialization (transient connection state)

### Position

```rust
pub struct Position {
    pub x: i32,  // World X coordinate
    pub y: i32,  // World Y coordinate
    pub z: i32,  // Plane/height level (0=ground, 1=first floor, etc)
}
```

## Packet Types

### Client → Server

```rust
ClientPacket::Login { username, password }
ClientPacket::Move { x, y }
ClientPacket::RequestPlayers
ClientPacket::RequestNpcs
ClientPacket::Chat { message }
ClientPacket::Ping { timestamp }
```

### Server → Client

```rust
ServerPacket::Welcome { message, tick }
ServerPacket::LoginSuccess { player_id, position, skills }
ServerPacket::PlayerEnter { id, username, position }
ServerPacket::PlayerLeave { id }
ServerPacket::PlayerMoved { id, position }
ServerPacket::PlayerList { players }
ServerPacket::NpcList { npcs }
ServerPacket::ChatMessage { username, message }
ServerPacket::Pong { timestamp, server_tick }
```

## Performance Characteristics

### Scalability

For **N** online players:
- **Login**: O(N) - must check visibility for all players
- **Movement**: O(N) - must check visibility before/after
- **Broadcast**: O(V) where V = visible players (typically << N)

### Optimization Opportunities

Current implementation is optimized for **simplicity** with moderate player counts (hundreds). For larger scale:

1. **Spatial Partitioning**: Use region grid to avoid checking all players
   - Already have `get_region_id()` and `get_nearby_regions()` functions
   - Could maintain per-region player lists
   - Would reduce visibility checks from O(N) to O(R) where R = players in nearby regions

2. **Interest Management**: Track which players each player can see
   - Maintain a visibility set per player
   - Only send updates when visibility set changes
   - Reduces enter/leave event overhead

3. **Message Batching**: Combine multiple updates into single messages
   - Reduces WebSocket overhead
   - Better for high-frequency updates

## Testing

The system includes comprehensive tests:

```rust
test_player_enters_view_distance()      // Player moves into range
test_player_leaves_view_distance()      // Player moves out of range
test_boundary_visibility()              // Exactly at 15-tile boundary
test_no_cross_plane_visibility()        // Different z-levels
test_visibility_with_multiple_players() // Complex scenarios
```

Run tests with:
```bash
cd src && cargo test
```

## Example Usage

### JavaScript Client

```javascript
// Login
ws.send(JSON.stringify({
    type: "Login",
    username: "PlayerName",
    password: "secret"
}));

// Handle enter/leave
ws.onmessage = (event) => {
    const packet = JSON.parse(event.data);
    
    switch(packet.type) {
        case "PlayerEnter":
            // Add player to visible list
            addPlayer(packet.id, packet.username, packet.position);
            break;
            
        case "PlayerLeave":
            // Remove player from visible list
            removePlayer(packet.id);
            break;
            
        case "PlayerMoved":
            // Update player position
            updatePlayerPosition(packet.id, packet.position);
            break;
    }
};
```

## Future Enhancements

1. **Ground Items**: Visibility-based item spawns
2. **Combat**: Only send damage/attack animations to nearby players
3. **NPCs**: Dynamic NPC loading based on player positions
4. **Instancing**: Separate visibility domains for different game instances
5. **Optimistic Updates**: Client-side prediction with server reconciliation

## Related Files

- `src/src/net/mod.rs` - Network handling and packet processing
- `src/src/world/mod.rs` - Visibility calculations
- `src/src/game/mod.rs` - Game state and player data
- `src/client/dist/index.html` - Test client implementation

## References

- [RuneScape Protocol](https://www.rune-server.ee/runescape-development/) - Original MMORPG protocol inspiration
- [Region-based Update Protocol](https://oldschool.runescape.wiki/w/Update_protocol) - OSRS wiki documentation