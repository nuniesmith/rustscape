# Rustscape Game Client

This directory contains the web-based game clients for Rustscape.

## 🎉 NEW: Reorganized Structure

**The client has been reorganized for better maintainability!**

### New Structure
```
client/
├── src/                    # Source code (JavaScript modules)
│   ├── main.js             # Entry point
│   ├── cache/              # Cache reading & parsing
│   ├── entities/           # Game entities (players, NPCs)
│   ├── rendering/          # 3D rendering utilities
│   └── utils/              # Utilities & constants
├── assets/                 # Static assets (CSS, images)
│   └── styles/             # Stylesheets
├── public/                 # Public files (HTML)
│   └── index.html          # 3D client entry point
└── dist/                   # Build output (gitignored)
```

### Documentation
- **STRUCTURE.md** - Detailed structure documentation
- **MIGRATION_GUIDE.md** - Migration from old structure
- **REORGANIZATION_SUMMARY.md** - Summary of changes

### Quick Start (3D Client)
```bash
cd client
npm install
npm run dev
# Open http://localhost:3000
```

---

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

### 🎮 3D Client (`/3d/`) - **NEWEST!**
The modern Three.js-based 3D client with:
- **Full 3D rendering** using Three.js
- **RuneScape Build 560 cache integration** - Real RS models!
- **Equipment system** with 11 slots
- **NPC AI** with wandering, idle, follow, hostile behaviors
- **Classic RuneScape UI** theme
- **Isometric camera** with zoom and rotation
- **Phase 3 features**:
  - PlayerModel class with multi-part body
  - Equipment hot-swapping
  - Animated NPCs
  - Name labels for players and NPCs

**Development Server:** `npm run dev` → `http://localhost:3000`

**See:** `STRUCTURE.md` for detailed documentation

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
├── src/                        # Source code (NEW!)
│   ├── main.js                 # 3D client entry point
│   ├── cache/                  # Cache system
│   │   ├── CacheReader.js      # Reads RS cache files
│   │   └── ModelParser.js      # Parses 3D models
│   ├── entities/               # Game entities
│   │   ├── PlayerModel.js      # Player with equipment
│   │   └── NPC.js              # NPCs with AI
│   ├── rendering/              # Rendering utilities
│   └── utils/                  # Utilities
│       └── constants.js        # Game configuration
├── assets/                     # Static assets (NEW!)
│   └── styles/
│       └── main.css            # RuneScape Classic theme
├── public/                     # Public files (NEW!)
│   └── index.html              # 3D client HTML
├── dist/                       # Legacy 2D clients
│   ├── index.html              # Landing page
│   ├── game.html               # 2D game client
│   ├── test-client.html        # Test client
│   └── 3d/                     # (OLD - being phased out)
├── package.json                # Dependencies
├── vite.config.js              # Build configuration
├── STRUCTURE.md                # Structure documentation
├── MIGRATION_GUIDE.md          # Migration guide
└── README.md                   # This file
```

## Development

### 3D Client (Modern)
```bash
cd client
npm install
npm run dev        # Start dev server
npm run build      # Build for production
```

**Structure:**
- Edit files in `src/`, `assets/`, `public/`
- Use path aliases: `@cache`, `@entities`, `@utils`
- Hot reload enabled
- See `STRUCTURE.md` for details

### 2D Clients (Legacy)
To modify the 2D clients:
1. Edit the HTML files in `client/dist/`
2. Refresh your browser to see changes (no build needed)
3. Use browser DevTools console for debugging

### Tips (2D Client)
- The game state is in the global `gameState` object (game.html)
- All rendering happens in the `render()` function
- WebSocket packets are handled in `handlePacket(packet)`
- The canvas uses a fixed tile size of 32px
- View range is 15 tiles in all directions

### Tips (3D Client)
- Import modules using path aliases: `import { X } from "@cache/X.js"`
- Game constants in `src/utils/constants.js`
- Entity classes in `src/entities/`
- Cache system in `src/cache/`
- See `STRUCTURE.md` for architecture details

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

### 2D Client
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

### 3D Client (Phase 3+)
Current focus:
- [ ] Real cache model loading (replace placeholders)
- [ ] Skeletal animation system
- [ ] Click-to-walk interaction
- [ ] Terrain from cache
- [ ] Multiplayer rendering
- [ ] Right-click context menus
- [ ] Combat system integration
- [ ] Sound effects

See `PROJECT_OVERVIEW.md` and `RESEARCH_GUIDE.md` for detailed roadmap.

---

## 📚 Additional Documentation

- **STRUCTURE.md** - Detailed structure documentation
- **MIGRATION_GUIDE.md** - How to migrate to new structure
- **REORGANIZATION_SUMMARY.md** - Summary of reorganization
- **README_3D.md** - 3D client specific documentation
- **../PROJECT_OVERVIEW.md** - Full project overview
- **../RESEARCH_GUIDE.md** - Learning resources
- **../NEXT_STEPS.md** - What to do next