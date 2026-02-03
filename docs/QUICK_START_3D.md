# Rustscape 3D Client - Quick Start Guide

**Get the 3D RuneScape-style client running in 5 minutes!**

---

## 🚀 Fastest Path to Running

### Step 1: Install Node.js Dependencies (1 minute)

```bash
cd ~/github/rustscape/client
npm install
```

**What this does:**
- Installs Three.js (3D rendering)
- Installs Vite (dev server)

**Expected output:**
```
added 12 packages in 15s
```

### Step 2: Start Rustscape Server (1 minute)

Open a **NEW terminal window**:

```bash
cd ~/github/rustscape
cargo run --release
```

**Wait for:**
```
🎮 Rustscape server running at http://localhost:8080
   WebSocket endpoint: ws://localhost:8080/ws
```

**Keep this terminal open!**

### Step 3: Start 3D Client (1 minute)

Back in the **first terminal**:

```bash
cd ~/github/rustscape/client
npm run dev
```

**Browser will auto-open to:** `http://localhost:3000`

---

## 🎮 You Should See

1. **Loading screen** with progress bar (2 seconds)
2. **3D isometric view** with:
   - Green terrain (wavy test terrain)
   - Gold blocky player character
   - Grid lines
   - "Player1" name label

3. **Right sidebar** with tabs:
   - ⚔️ Combat
   - 📊 Stats (active)
   - 📜 Quest
   - 🎒 Inventory
   - 👕 Equipment
   - 🙏 Prayer
   - ✨ Magic

4. **Top-right corner:**
   - Circular minimap (black for now)
   - Compass (N)
   - 3 XP orbs (HP, Prayer, Run)

5. **Bottom:** Chat box

6. **Top-left:** Green "FPS: 60" counter

---

## ✅ It's Working If...

- ✅ You see "FPS: 60" (green text)
- ✅ You can rotate camera (arrow keys)
- ✅ You can zoom in/out (mouse wheel)
- ✅ Chat shows "Welcome to Rustscape!"
- ✅ Tabs switch when clicked

---

## 🛑 If Something's Wrong

### Black Screen?
```bash
# Check browser console (F12)
# Look for errors
# Try Chrome if using another browser
```

### "Cannot find module 'three'"?
```bash
cd ~/github/rustscape/client
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Server not running?
```bash
# Make sure you ran:
cd ~/github/rustscape
cargo run --release
# in a SEPARATE terminal
```

### Port already in use?
```bash
# Kill existing process:
killall -9 vite
# or change port in vite.config.js
```

---

## 🎯 Try These Controls

### Keyboard
- **Arrow Left/Right** - Rotate camera
- **Type in chat** - Press Enter to send

### Mouse
- **Scroll wheel** - Zoom in/out
- **Click tabs** - Switch interface panels

### Chat Commands
Type these in the chat box:

```
/help       - Show available commands
/fps        - Toggle FPS counter
/connect    - Connect to server (coming soon)
```

---

## 📊 What You're Seeing (Explained)

### Current Implementation (Phase 1 - Complete ✅)

1. **3D Scene**
   - Three.js WebGL renderer
   - Isometric orthographic camera
   - Ambient + directional lighting
   - Test terrain (100×100 grid with sine wave heights)
   - Simple player model (gold boxes)

2. **UI**
   - Classic RuneScape brown/gold theme
   - 7-tab interface
   - Inventory (28 slots)
   - Equipment (11 slots)
   - Chat with tabs
   - XP orbs
   - Minimap container

3. **Controls**
   - Camera rotation (4 angles: N, E, S, W)
   - Zoom (0.5x to 2.0x)
   - FPS counter
   - Chat input

### Not Yet Implemented (Phase 2-3)

- ❌ Cache data loading (models from Build 560)
- ❌ Real player models
- ❌ Actual terrain from cache
- ❌ Equipment rendering
- ❌ NPCs
- ❌ Animations
- ❌ Click-to-walk
- ❌ Server connection

**This is a foundation!** The architecture is ready, now we add features.

---

## 🔧 Development Workflow

### Making Changes

1. **Edit a file** (e.g., `client/dist/3d/js/main.js`)
2. **Save**
3. **Browser auto-reloads** (Vite hot reload)
4. **See changes instantly**

### Debugging

Press **F12** in browser to open DevTools:

```javascript
// In console, access game state:
window.gameState

// Inspect:
gameState.scene       // Three.js scene
gameState.camera      // Camera object
gameState.renderer    // WebGL renderer
gameState.player      // Player mesh
```

### Building for Production

```bash
npm run build
# Output: client/build/
```

---

## 📁 Key Files to Know

| File | Purpose |
|------|---------|
| `client/dist/3d/index.html` | Main HTML, UI layout |
| `client/dist/3d/js/main.js` | Entry point, Three.js setup |
| `client/package.json` | Dependencies |
| `client/vite.config.js` | Dev server config |

---

## 🎨 Customization Quick Tips

### Change Terrain Color

Edit `main.js`, line ~167:
```javascript
color: 0x3a6b1f, // Change this hex color
```

### Change Player Color

Edit `main.js`, line ~199:
```javascript
color: 0xcc9900, // Gold - change to any hex
```

### Change Camera Angle

Edit `main.js`, line ~135:
```javascript
camera.position.set(15, 15, 15); // X, Y, Z
```

### Add More Test Objects

In `main.js`, after `createTestPlayer()`:
```javascript
// Add a box
const geometry = new THREE.BoxGeometry(1, 1, 1);
const material = new THREE.MeshLambertMaterial({ color: 0xff0000 });
const cube = new THREE.Mesh(geometry, material);
cube.position.set(5, 0.5, 5);
scene.add(cube);
```

---

## 📚 Next Steps

1. **✅ You have the foundation running**
2. **📖 Read:** `docs/3D_CLIENT_SPECIFICATION.md` (full spec)
3. **🎯 Next Phase:** Implement cache reader
4. **💬 Questions?** Check the docs in `rustscape/docs/`

---

## 🚀 Phase 2 Preview (Coming Next)

What we'll build in the next 2 weeks:

1. **Cache Reader**
   - Parse Build 560 cache files
   - Extract models, textures, sprites

2. **Real Models**
   - Load actual RuneScape player models
   - Equipment rendering
   - NPC models

3. **Real Terrain**
   - Load terrain from cache
   - Proper textures
   - Height maps

4. **Server Integration**
   - WebSocket connection
   - Sync player position
   - See other players

---

## 🎮 Current vs Target Look

**Current (Phase 1):**
```
Simple colored boxes
Test terrain with grid
Basic UI layout
```

**Target (Phase 4):**
```
Authentic RuneScape models from cache
Realistic terrain with textures
Full equipment rendering
Animations (walk, attack)
Click-to-walk
Multiplayer
```

**We're 25% there!** Foundation is solid, now we add the RuneScape magic.

---

## 💡 Tips

- **Keep server running** while developing client
- **Use Chrome** for best WebGL performance
- **Check FPS counter** - should be green (60 FPS)
- **Save often** - Vite hot reload is instant
- **Read console** - errors show there first

---

## ✅ Success Checklist

- [x] Node.js installed
- [x] `npm install` completed
- [x] Server running (`cargo run`)
- [x] Client running (`npm run dev`)
- [x] Browser shows 3D scene
- [x] FPS: 60 (green)
- [x] Can rotate camera
- [x] Can zoom
- [x] UI tabs work
- [x] Chat works

**All checked?** You're ready to build! 🎉

---

**Questions?** Check `client/README_3D.md` or `docs/` folder.

**Ready to code?** Edit `client/dist/3d/js/main.js` and see live changes!