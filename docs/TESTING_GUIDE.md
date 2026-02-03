# Testing Guide - Graphics Update

## Quick Start Testing

### 1. Build and Run
```bash
cd ~/github/rustscape
cargo run --release
```

Expected output:
```
INFO rustscape: 🎮 Rustscape server running at http://localhost:8080
INFO rustscape:    WebSocket endpoint: ws://localhost:8080/ws
```

### 2. Open the Game Client
Open your browser and navigate to:
```
http://localhost:8080/game.html
```

### 3. Login
- Default username: `Player1`
- Default password: `password`
- Click "Login"

## What to Test

### ✅ Player Name Display (FIXED)
**Before**: Local player showed "YOU"  
**After**: Local player shows actual username (e.g., "Player1")

**How to verify:**
1. Login with username "Player1"
2. Look at the character in the center of the screen
3. You should see "Player1" above the gold/yellow sprite (NOT "YOU")

### ✅ OSRS-Style Graphics (NEW)
**What changed**: All sprites now have OSRS-style rendering

**How to verify:**

#### Terrain
- **Grass**: Dark green (#2d5016) with subtle grid borders
- **Dirt paths**: Brown with small texture dots
- **Water**: Blue with animated wave patterns (curved lines)
- **Trees**: Brown trunk with multi-layered green canopy
- **Rocks**: Gray with 3-layer shading (highlight, base, shadow)

#### Your Character
- **Color**: Gold/yellow (#cc9900)
- **Highlight**: Bright 6x6 pixel square in top-left
- **Shadow**: Dark oval beneath sprite
- **Outline**: 1px black border around sprite
- **Name**: White text "Player1" with black shadow effect

#### Other Features
- All text has shadow blur for readability
- Entities have drop shadows
- Clear sprite outlines
- Color-coded entity types

### 🎮 Gameplay Testing

#### Movement
1. Click on grass tiles around your character
2. Character should move to clicked location
3. Camera should follow your character (keeping it centered)
4. Username should stay with your character

#### NPCs
1. Look for orange/red NPCs (labeled with yellow names)
2. NPCs should have:
   - Orange sprite with highlight
   - Black outline
   - Drop shadow beneath
   - Yellow name above
   - Green/orange/red HP bar below (if in combat)

#### Multi-Player (Optional)
1. Open a second browser tab
2. Login as "Player2"
3. In first tab, you should see Player2 as a cyan/blue sprite
4. In second tab, you should see Player1 as a cyan/blue sprite
5. Each tab shows their own character as gold/yellow with their username

#### Ground Items
1. Look for small amber/gold squares on the ground
2. Items should have:
   - 8x8 sprite (smaller than characters)
   - Orange/amber color
   - Drop shadow
   - White name label below

## Visual Checklist

Use this checklist to verify the graphics update:

- [ ] Local player shows correct username (not "YOU")
- [ ] Local player is gold/yellow color
- [ ] Other players are cyan/blue color
- [ ] NPCs are orange/red color
- [ ] All sprites have drop shadows
- [ ] All sprites have black outlines
- [ ] All sprites have top-left highlights
- [ ] Text has shadow blur effect
- [ ] Trees have brown trunks and green canopy
- [ ] Rocks have shading (light/dark areas)
- [ ] Water has wave pattern lines
- [ ] Dirt paths have texture dots
- [ ] All tiles have subtle grid borders
- [ ] HP bars show on NPCs during combat
- [ ] HP bars are color-coded (green/orange/red)

## Common Issues

### Issue: Player still shows "YOU"
**Solution**: Hard refresh the browser (Ctrl+F5 or Cmd+Shift+R)

### Issue: Graphics look blocky/pixelated
**Expected**: This is the OSRS-style aesthetic! It's intentional.

### Issue: Can't see grid borders
**Expected**: Borders are subtle (15% opacity). Look closely at tile edges.

### Issue: Text is hard to read
**Check**: All text should have black shadow blur. If not, refresh browser.

### Issue: Colors don't match this guide
**Solution**: 
1. Clear browser cache
2. Hard refresh (Ctrl+F5)
3. Restart server if needed

## Performance Testing

### Frame Rate
- Game should render smoothly without stuttering
- Canvas should resize properly when window is resized
- No lag when moving or clicking

### Multiple Entities
- Test with multiple NPCs visible (13 NPCs spawn by default)
- All should render correctly
- No performance degradation

### Large View Range
- 31x31 tiles visible (961 tiles total)
- All terrain should render correctly
- No missing tiles or gaps

## Browser Compatibility

Test in multiple browsers if possible:

- [ ] Chrome/Chromium (recommended)
- [ ] Firefox
- [ ] Edge
- [ ] Safari

All should work identically.

## Comparison: Before vs After

### Before (Old Graphics)
- Player showed "YOU" label
- Simple colored squares
- No shadows
- No outlines
- Flat terrain colors
- No texture details
- Hard to read text

### After (New Graphics)
- Player shows actual username
- Sprite-style rendering
- Drop shadows on all entities
- Black outlines for definition
- Textured terrain (trees, rocks, water waves)
- Shadow blur on all text
- OSRS-inspired aesthetic

## Screenshots Recommended

Take screenshots of:
1. Your character (gold sprite with username)
2. Another player (cyan sprite)
3. An NPC (orange sprite with HP bar)
4. Terrain variety (grass, water, trees, rocks)
5. Ground items

Compare with earlier screenshots to see the improvement!

## Next Steps After Testing

If everything works:
1. ✅ Graphics update successful!
2. Ready for gameplay features (Tutorial Island, quests, etc.)
3. Consider adding sprite images to replace geometric shapes

If issues found:
1. Check browser console for errors (F12)
2. Verify server is running latest code
3. Hard refresh browser
4. Check `GRAPHICS_UPDATE.md` for detailed specs

## Documentation

For more details, see:
- `GRAPHICS_UPDATE.md` - Technical specifications
- `docs/VISUAL_GUIDE.md` - Visual reference and color palette
- `docs/CHANGELOG_2024-02-03.md` - Complete changelog
- `client/README.md` - Client features and controls

## Feedback

What to look for:
- Does it feel like OSRS?
- Are the colors distinct and readable?
- Is text always visible?
- Do sprites look good at different resolutions?
- Are shadows helping or distracting?

The graphics system is designed to be:
- **Authentic**: Matches OSRS aesthetic
- **Readable**: Clear entity identification
- **Performant**: Smooth rendering
- **Scalable**: Easy to add new entity types

Enjoy testing the new graphics! 🎮