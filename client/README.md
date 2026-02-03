# Rustscape Game Client

This directory contains the web-based game clients for Rustscape.

## Available Clients

### 🎮 Main Game Client (`game.html`)
The modern, feature-rich game client with:
- **2D Canvas rendering** with tile-based top-down view
- **RuneScape-inspired UI** with proper panels and styling
- **Interactive gameplay**:
  - Click-to-move on the game world
  - Click NPCs to attack them
  - Click ground items to pick them up
  - Real-time combat with visual feedback
- **UI Components**:
  - Player stats panel (position, combat level, HP bar)
  - Skills panel (Attack, Strength, Defence, Hitpoints)
  - Inventory grid (28 slots, click to drop items)
  - Chat box with color-coded messages
- **Visual Features**:
  - Player character (yellow square - "YOU")
  - Other players (blue squares with usernames)
  - NPCs (orange squares with names and HP bars)
  - Ground items (orange dots with item names)
  - Hover tooltips showing entity information
  - Level-up notifications
  - XP gain messages

**Access at:** `http://localhost:8080/game.html`

### 🧪 Test Client (`test-client.html`)
The original debug/test client with:
- Manual button controls for all game actions
- Detailed packet logging
- Console-based debugging
- Useful for development and testing

**Access at:** `http://localhost:8080/test-client.html`

### 🏠 Index Page (`index.html`)
Landing page that auto-redirects to the main game client after 2 seconds.

**Access at:** `http://localhost:8080/` or `http://localhost:8080/index.html`

## Building

The clients are static HTML files with embedded JavaScript and CSS. No build step is required!

Just run the Rustscape server:
```bash
cd rustscape/src
cargo run
```

Then open your browser to `http://localhost:8080`

## Game Controls

### Main Game Client
- **Left Click on Tile**: Move to that position
- **Left Click on NPC**: Attack the NPC
- **Left Click on Ground Item**: Pick up the item
- **Inventory Slot Click**: Drop the item at current position
- **Chat Input**: Type and press Enter to send messages
- **Hover**: View information about NPCs and items

### Test Client
All actions are performed through buttons in the UI panels.

## Protocol

Both clients communicate with the server via WebSocket using JSON packets.

**WebSocket Endpoint:** `ws://localhost:8080/ws`

### Key Client Packets
- `Login` - Authenticate with username/password
- `Move` - Request movement to a position
- `Attack` - Attack a player or NPC
- `Chat` - Send chat message
- `PickupItem` - Pick up ground item
- `DropItem` - Drop inventory item
- `TalkToNpc` - Interact with NPC
- `RequestPlayers/Npcs/GroundItems` - Request entity lists

### Key Server Packets
- `LoginSuccess` - Login confirmation with player data
- `PlayerMoved` - Player position update
- `CombatHit` - Combat damage event
- `XpGain` - Experience points gained
- `LevelUp` - Skill level increased
- `GroundItemSpawned/Removed` - Item on ground
- `ChatMessage` - Chat message broadcast
- `NpcDialogue` - NPC talking

## File Structure

```
client/
├── dist/                    # Served by the game server
│   ├── index.html          # Landing page (auto-redirects)
│   ├── game.html           # Main game client
│   └── test-client.html    # Debug/test client
└── README.md               # This file
```

## Development

To modify the clients:
1. Edit the HTML files in `client/dist/`
2. Refresh your browser to see changes (no build needed)
3. Use browser DevTools console for debugging

### Tips
- The game state is in the global `gameState` object (game.html)
- All rendering happens in the `render()` function
- WebSocket packets are handled in `handlePacket(packet)`
- The canvas uses a fixed tile size of 32px
- View range is 15 tiles in all directions

## Browser Compatibility

Works on modern browsers with:
- WebSocket support
- Canvas 2D rendering
- ES6+ JavaScript

Tested on:
- Chrome/Edge 90+
- Firefox 88+
- Safari 14+

## Future Enhancements

Potential improvements:
- [ ] Sprite-based graphics instead of colored squares
- [ ] Minimap showing larger area
- [ ] Equipment panel with paper doll
- [ ] Right-click context menus
- [ ] Keyboard shortcuts (WASD movement)
- [ ] Sound effects and music
- [ ] Mobile touch controls
- [ ] Admin/debug panel
- [ ] Skills progress bars with XP to next level
- [ ] Combat animations (hit splats, projectiles)