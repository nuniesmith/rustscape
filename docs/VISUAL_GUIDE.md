# Visual Guide - Rustscape OSRS-Style Graphics

## Overview

This document provides a visual reference for the new OSRS-style graphics rendering system implemented in Rustscape.

## Entity Visual Reference

### Player Characters

#### Local Player (You)
```
Color Scheme: Gold/Yellow
- Base: #cc9900 (dark gold)
- Highlight: #ffcc00 (bright gold)
- Outline: #000000 (black, 1px)
- Shadow: rgba(0,0,0,0.4) - positioned below sprite
- Name: White (#ffffff) with 2px black shadow blur
- Size: 16x16 pixels (within 32px tile)
```

Visual representation:
```
     [Player1]  ← White text with shadow
    ┌────────┐
    │■■░░░░  │  ← 6x6 bright highlight (top-left)
    │■░░░░░  │
    │■░░░░░  │  ← 16x16 gold sprite
    │░░░░░░  │
    │░░░░░░  │
    └────────┘
     ▓▓▓▓▓▓   ← Drop shadow
```

#### Other Players
```
Color Scheme: Cyan/Blue
- Base: #0088cc (dark cyan)
- Highlight: #00aaff (bright cyan)
- Outline: #000000 (black, 1px)
- Shadow: rgba(0,0,0,0.4)
- Name: White (#ffffff) with shadow
- Same size as local player
```

### NPCs (Non-Player Characters)

```
Color Scheme: Orange/Red
- Base: #cc5500 (dark orange)
- Highlight: #ff6600 (bright orange)
- Outline: #000000 (black, 1px)
- Shadow: rgba(0,0,0,0.4)
- Name: Yellow (#ffff00) with shadow
- HP Bar: 5px tall, positioned 2px below sprite
```

Visual representation:
```
      [Goblin]  ← Yellow text with shadow
    ┌────────┐
    │■■░░░░  │  ← Orange sprite with highlight
    │■░░░░░  │
    │░░░░░░  │
    │░░░░░░  │
    └────────┘
    ╔════════╗  ← Black background
    ║████░░░░║  ← HP fill (50% = orange)
    ╚════════╝
     ▓▓▓▓▓▓   ← Drop shadow
```

HP Bar Color Coding:
- **Green** (#00ff00): HP > 50%
- **Orange** (#ffaa00): HP 25-50%
- **Red** (#ff0000): HP < 25%

### Ground Items

```
Color Scheme: Amber/Gold
- Base: #cc8800 (dark amber)
- Highlight: #ffaa00 (bright amber)
- Outline: #000000 (black, 1px)
- Shadow: rgba(0,0,0,0.3)
- Name: White (#ffffff) with shadow
- Size: 8x8 pixels (smaller than characters)
```

Visual representation:
```
    ┌──┐
    │■░│  ← 4x4 highlight
    │░░│  ← 8x8 item sprite
    └──┘
     ▓▓  ← Drop shadow
  [Coins]  ← Name below
```

## Terrain Types

### Grass (Default)
```
Color: #2d5016 (dark green)
Border: rgba(0,0,0,0.15) - subtle grid lines
Details: Flat color with border
```

### Dirt Path
```
Color: #8b6f47 (brown)
Border: rgba(0,0,0,0.15)
Details: Small texture dots scattered
  - 2x2 pixel dots at random positions
  - 10% opacity black for texture
```

### Water
```
Color: #1a4d7a (deep blue)
Border: rgba(0,0,0,0.15)
Details: Curved wave lines
  - Stroke color: #2a5d9b (lighter blue)
  - Line width: 2px
  - Multiple wave layers at different heights
```

Visual pattern:
```
┌────────────────────────────┐
│     ∼∼∼∼∼∼                │
│         ∼∼∼∼∼∼            │  ← Wave lines
│             ∼∼∼∼∼∼        │
└────────────────────────────┘
```

### Stone
```
Color: #666666 (gray)
Border: rgba(0,0,0,0.15)
Details: Flat color with border
```

### Trees
```
Base: Same as grass
Tree trunk: #6b4423 (brown) - 6px wide, 9px tall
Canopy: Multi-layered circles
  - Dark base: #2d5016 (9px radius)
  - Light highlight: #3a6b1f (4px radius, offset)
```

Visual structure:
```
      ●●●    ← Light green highlight
    ●●●●●●●  ← Dark green canopy (circle)
    ●●●●●●●
      ║║║    ← Brown trunk (rectangle)
      ║║║
```

### Rocks
```
Base: Same as grass
Rock: Multi-layered circles for depth
  - Base: #757575 (9px radius)
  - Shadow: #555555 (5px radius, offset bottom-right)
  - Highlight: #8a8a8a (3px radius, offset top-left)
```

Visual structure:
```
    ○         ← Light gray highlight
  ●●●●●       ← Base gray (9px)
 ●●●●●●●
 ●●●●●●●
  ●●●●●       
    ●●        ← Dark gray shadow
```

## UI Color Palette

### Text Colors
- **Server messages**: #ff8800 (orange)
- **Player chat**: #00ffff (cyan)
- **NPC dialogue**: #ffff00 (yellow)
- **Combat events**: #ff0000 (red)
- **XP/Level-up**: #00ff00 (green)
- **Entity names (players)**: #ffffff (white)
- **Entity names (NPCs)**: #ffff00 (yellow)
- **Entity names (items)**: #ffffff (white)

### Panel Colors
- **Background**: linear-gradient(#3a3a3a to #2a2a2a)
- **Border**: #000000 (2px solid)
- **Headers**: #ffcc00 (gold)
- **Text**: #ffffff (white)
- **Stat values**: #00ff00 (green)

### HP Bar
- **Background**: #000000 (black)
- **Border**: #000000 (black, 1px)
- **Fill colors**:
  - High (>50%): #00ff00 (green)
  - Medium (25-50%): #ffaa00 (orange)
  - Low (<25%): #ff0000 (red)

## Rendering Order (Z-Index)

Entities are rendered in this order (bottom to top):

1. **Background** (#0a0a0a solid fill)
2. **Terrain tiles** (grass, water, stone, etc.)
3. **Terrain details** (trees, rocks on top of grass)
4. **Ground items** (coins, equipment on ground)
5. **NPCs** (monsters, friendly NPCs)
6. **Other players** (all other connected players)
7. **Local player** (you - always rendered last/on top)
8. **HP bars** (above NPCs)
9. **Text labels** (names above entities)

## Typography

### Font Face
- **Primary**: "Courier New", monospace (bold)
- **Size**: 9-10px for entity names
- **Style**: Bold weight for all text
- **Effects**: 2px black shadow blur on all entity names

### Text Shadow Effect
```javascript
ctx.shadowColor = "#000000";
ctx.shadowBlur = 2;
ctx.fillText(text, x, y);
ctx.shadowBlur = 0; // Reset after each text
```

## Tile System

### Tile Size
- **Pixels**: 32x32 pixels per tile
- **View range**: 15 tiles in each direction (31x31 visible area)
- **Camera**: Centered on local player
- **Grid**: Subtle black borders (15% opacity)

### Coordinate System
```
World coordinates: (x, y, z)
- X increases east →
- Y increases south ↓
- Z is vertical (not currently used in rendering)

Screen coordinates: Relative to player
- (0,0) = player position (center of screen)
- Tiles rendered from (playerX-15, playerY-15) to (playerX+15, playerY+15)
```

## Performance Specifications

### Target Performance
- **Frame rate**: Rendering loop runs at render() call intervals
- **View range**: 31x31 tiles (961 tiles max)
- **Entity limit**: No hard limit, renders all visible entities
- **Canvas size**: Responsive to window size

### Optimization Techniques
1. Simple geometric shapes (rectangles, circles)
2. Minimal fill operations per tile
3. Efficient shadow rendering (single rectangle per entity)
4. Text rendered last to avoid overdraw
5. Grid borders drawn once per tile

## Browser Compatibility

Tested and working on:
- ✅ Chrome/Chromium (latest)
- ✅ Firefox (latest)
- ✅ Edge (latest)
- ✅ Safari (latest)

Requirements:
- HTML5 Canvas support
- WebSocket support
- ES6 JavaScript support

## Animation Notes (Future)

Current: Static sprites with no animation
Future enhancements planned:
- Walking cycles (4-8 frames)
- Directional facing (N, S, E, W)
- Attack animations
- Hit splats and damage numbers
- Spell effects and projectiles
- Idle animations

## Accessibility

Current implementation:
- High contrast colors for readability
- Shadow effects on all text for visibility against any background
- Clear entity outlines for definition
- Color-coded HP bars with gradual transitions

Future improvements:
- Colorblind mode options
- Adjustable UI scaling
- Screen reader support for chat
- Keyboard navigation

## Developer Notes

### Rendering Pipeline
```
1. Clear canvas (solid #0a0a0a background)
2. Calculate visible tile range based on player position
3. For each visible tile:
   a. Draw terrain base color
   b. Draw terrain details (trees, rocks, water waves)
   c. Draw tile border
4. Draw ground items (with shadows, sprites, names)
5. Draw NPCs (with shadows, sprites, HP bars, names)
6. Draw other players (with shadows, sprites, names)
7. Draw local player (with shadow, sprite, name)
```

### Adding New Entity Types

To add a new renderable entity:

1. Choose a unique color scheme
2. Add drop shadow (rgba(0,0,0,0.4))
3. Add 16x16 base sprite
4. Add 6x6 top-left highlight
5. Add 1px black outline
6. Add text label with shadow blur
7. Position in correct render order

Example template:
```javascript
// Shadow
ctx.fillStyle = "rgba(0, 0, 0, 0.4)";
ctx.fillRect(screenX + 10, screenY + TILE_SIZE - 6, TILE_SIZE - 20, 3);

// Base sprite
ctx.fillStyle = "#YOUR_BASE_COLOR";
ctx.fillRect(screenX + 8, screenY + 8, TILE_SIZE - 16, TILE_SIZE - 16);

// Highlight
ctx.fillStyle = "#YOUR_HIGHLIGHT_COLOR";
ctx.fillRect(screenX + 9, screenY + 9, 6, 6);

// Outline
ctx.strokeStyle = "#000000";
ctx.lineWidth = 1;
ctx.strokeRect(screenX + 8, screenY + 8, TILE_SIZE - 16, TILE_SIZE - 16);

// Name
ctx.fillStyle = "#YOUR_TEXT_COLOR";
ctx.font = "bold 10px Courier New";
ctx.textAlign = "center";
ctx.shadowColor = "#000000";
ctx.shadowBlur = 2;
ctx.fillText(name, screenX + TILE_SIZE / 2, screenY - 5);
ctx.shadowBlur = 0;
```

## Credits

Graphics style inspired by:
- Old School RuneScape (OSRS)
- RuneScape Classic (RSC)
- Early 2000s MMORPGs

Implemented with:
- HTML5 Canvas API
- Vanilla JavaScript (ES6)
- No external graphics libraries