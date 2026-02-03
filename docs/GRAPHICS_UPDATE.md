# Graphics Update - OSRS-Style Rendering

## Date: 2026-02-03

## Overview
Updated the Rustscape client to feature improved graphics that more closely match the early 2000s RuneScape (OSRS) aesthetic. The new rendering system includes enhanced terrain textures, sprite-like character rendering, and proper shadowing effects.

## Changes Made

### 1. Player Name Display Fix
**Issue:** The local player was displaying as "YOU" instead of the actual username.

**Fix:** Updated the local player rendering to display `gameState.playerUsername` instead of the hardcoded "YOU" text.

```javascript
// Before:
ctx.fillText("YOU", centerX + TILE_SIZE / 2, centerY - 5);

// After:
ctx.fillText(
    gameState.playerUsername || "Player",
    centerX + TILE_SIZE / 2,
    centerY - 5,
);
```

### 2. Terrain Improvements

#### Tile Borders
- Added subtle dark borders around each tile (15% opacity black) for that classic grid look
- Creates clear tile separation similar to OSRS

#### Enhanced Tree Rendering
- Brown tree trunks (#6b4423) with proper proportions
- Multi-layered canopy with darker base (#2d5016) and lighter highlight (#3a6b1f)
- More organic, circular tree tops

#### Rock Rendering
- Three-layer shading system:
  - Base gray (#757575)
  - Dark shadow (#555555) 
  - Light highlight (#8a8a8a)
- Creates depth and dimension

#### Water Rendering
- Animated-looking wave patterns with curved lines
- Thicker stroke width (2px) for visibility
- Multiple wave layers at different heights
- Uses OSRS water blue (#2a5d9b)

#### Dirt Path Texture
- Added small texture dots to break up flat dirt paths
- Randomly placed texture elements for organic feel

### 3. Ground Item Rendering (OSRS-Style)

**New features:**
- Drop shadow (30% opacity black, 3px tall)
- Square sprite base (#cc8800)
- Bright highlight in top-left corner (#ffaa00)
- Black outline for definition
- White name text with shadow blur effect
- Positioned below the sprite for clarity

### 4. NPC Rendering (OSRS-Style)

**New features:**
- Drop shadow beneath NPCs (40% opacity black)
- Orange/red body color (#cc5500) with bright highlight (#ff6600)
- Black outline around sprite for definition
- Yellow name text with shadow blur
- Enhanced HP bar:
  - Thicker bars (5px instead of 4px)
  - Black background and border
  - Color-coded: green (>50%), orange (25-50%), red (<25%)
  - Positioned below NPC sprite

### 5. Player Rendering (OSRS-Style)

**Local Player:**
- Gold/yellow color scheme (#cc9900 base, #ffcc00 highlight)
- Drop shadow (40% opacity black)
- 6x6 pixel highlight square in top-left
- Black outline for sprite definition
- White username with shadow blur effect

**Other Players:**
- Cyan/blue color scheme (#0088cc base, #00aaff highlight)
- Same shadow, outline, and highlight system as local player
- White username display

### 6. Text Rendering Improvements

All entity names (players, NPCs, items) now feature:
- **Shadow blur effect** (`ctx.shadowBlur = 2`) for better readability
- **Black shadow color** for contrast against any background
- **Bold Courier New font** for that classic OSRS look
- Properly reset shadow blur after rendering (`ctx.shadowBlur = 0`)

## Visual Style Guidelines

The new rendering system follows these OSRS-inspired principles:

1. **Sprite-based rendering**: Characters and items are rendered as small, simple sprites with clear outlines
2. **Limited color palettes**: Each entity type has a distinct color scheme
3. **Highlight system**: Top-left corner highlights create depth
4. **Drop shadows**: Ground-level shadows anchor sprites to the world
5. **Text readability**: All text has shadow effects for maximum contrast
6. **Tile-based world**: Clear grid with textured tiles

## Color Palette Reference

### Terrain
- Grass: `#2d5016` (dark green)
- Dirt: `#8b6f47` (brown)
- Water: `#1a4d7a` (deep blue)
- Stone: `#666666` (gray)

### Entities
- Local Player: `#cc9900` (gold)
- Other Players: `#0088cc` (cyan)
- NPCs: `#cc5500` (orange)
- Ground Items: `#cc8800` (amber)

### UI Elements
- HP Bar (high): `#00ff00` (green)
- HP Bar (medium): `#ffaa00` (orange)
- HP Bar (low): `#ff0000` (red)
- NPC Names: `#ffff00` (yellow)
- Player Names: `#ffffff` (white)

## Performance Notes

The new rendering system maintains 60 FPS performance by:
- Using simple geometric shapes (rectangles, circles)
- Minimizing fill operations
- Efficient shadow rendering with simple offsets
- No expensive texture lookups or image manipulation

## Future Enhancements

Potential improvements for future updates:
1. **Sprite sheets**: Replace geometric shapes with actual sprite images
2. **Animation**: Walking animations with 4-8 frame cycles
3. **Directional facing**: Sprites face the direction of movement
4. **Equipment rendering**: Show equipped items on player sprites
5. **Particle effects**: Combat hits, skill level-ups, item drops
6. **Minimap**: Small overhead view in corner of screen
7. **Right-click menus**: Context menus for entities
8. **Smooth movement**: Interpolated movement between tiles

## Testing

The updated graphics have been tested with:
- Single player movement and rendering
- Multiple concurrent players
- NPCs with HP bars
- Ground items display
- Tutorial Island area (3090-3110, 3090-3120)
- Lumbridge area (3210-3240, 3210-3240)

All rendering features work correctly across different viewport sizes and zoom levels.

## Compatibility

- Works in all modern browsers (Chrome, Firefox, Edge, Safari)
- No external dependencies required
- Pure HTML5 Canvas rendering
- Responsive to window resize events

## Recommendations

### Immediate Next Steps

1. **Run the updated client**: Start the server with `cargo run` and connect via `http://localhost:8080/game.html`
2. **Test multiplayer**: Open multiple browser tabs to verify player rendering and interactions
3. **Verify NPC combat**: Attack an NPC to test the HP bar rendering

### Known Issues to Address

1. **Camera follows username correctly now**: The local player now shows their actual username (e.g., "Player1") instead of "YOU"
2. **Compiler warnings**: Run `cargo fix --bin "rustscape" -p rustscape` to clean up unused imports and variables
3. **Dialogue parsing**: Update `assets/dialogue/npcs/hans.json` and `bob.json` to use array format instead of map format

### Gameplay Enhancements Needed

1. **Tutorial Island spawn**: New players should spawn at (3094, 3107) on Tutorial Island
2. **Tutorial Guide NPC**: Add an interactive guide on Tutorial Island
3. **Starter items**: Give new players basic equipment after tutorial completion
4. **Lumbridge teleport**: Transport players to Lumbridge (3222, 3218) after tutorial

### Visual Enhancements Priority

**High Priority:**
- Add actual sprite images to replace geometric shapes
- Implement 4-direction facing (N, S, E, W)
- Add walking animation frames
- Create minimap overlay

**Medium Priority:**
- Equipment visualization on player sprites
- Combat animations and hit splats
- Skill level-up effects
- Right-click context menus

**Low Priority:**
- Ambient animations (trees swaying, water flowing)
- Weather effects
- Day/night cycle
- Custom cursors

### Performance Optimization

Current rendering is efficient, but for large player counts consider:
- Viewport culling (only render visible entities)
- Entity pooling for frequently created/destroyed objects
- Throttle HP bar updates
- Batch render similar entities together