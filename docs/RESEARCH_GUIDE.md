# 🔬 Rustscape Research Guide - What to Study Next

**Purpose:** Focused research recommendations to accelerate Phase 3+ development  
**Time Investment:** ~20 hours of research over next 2 weeks  
**Priority:** Immediate (blocking current features)

---

## 🎯 Critical Path Research (Do First!)

### 1. RuneScape Cache Format - HIGHEST PRIORITY

**Time Investment:** 6-8 hours  
**Why Critical:** Blocking real model/animation loading

#### What to Study

**Cache Archive Structure:**
```
Build 560 Cache Layout:
├── Archive 0: Animation Sequences (.dat + .idx)
├── Archive 1: Skeleton/Bone Data
├── Archive 7: Model Geometry
└── Archive 5: Map/Terrain Data

Each archive has:
- .dat file (raw data)
- .idx file (index/offsets)
```

**Specific Topics:**
1. **Model Format (Archive 7)**
   - Vertex data encoding
   - Face/triangle definitions
   - Color indexing system
   - Texture coordinates
   - Bone weights (for animated models)

2. **Animation Format (Archive 0)**
   - Keyframe structure
   - Bone transformation data
   - Frame timing/duration
   - Animation loops vs one-shots

3. **Skeleton Format (Archive 1)**
   - Bone hierarchy (parent/child)
   - Bone IDs and names
   - Default bone positions
   - Attachment points (for equipment)

#### Learning Resources

**Primary Sources:**
- OpenRS2 Archive: https://archive.openrs2.org/
  - Download cache files
  - Study format documentation
  - Browse existing tools

- RuneScape Wiki (OSRS):
  - Item IDs and model references
  - NPC definitions
  - Animation IDs

**Code References:**
- `RSCacheEditor` (Java) - Open source cache viewer
  - Study model loading code
  - See how animations are parsed
  - GitHub: search "rs cache editor"

- `OpenRS` (Multiple implementations)
  - Cache reading libraries
  - Model parsers
  - Animation systems

**Hex Editor Practice:**
- Download HxD or similar
- Open a .dat file from Archive 7
- Identify patterns (vertices, faces, colors)
- Map byte offsets to data structures

#### Action Steps

**Day 1-2: Model Format**
```
1. Download Build 560 cache
2. Extract Archive 7 (models)
3. Pick a simple model (ID: 0-100)
4. Open in hex editor
5. Identify:
   - Vertex count (usually first 2 bytes)
   - Face count
   - Color data
6. Update ModelParser.js to handle real data
```

**Day 3-4: Animation Format**
```
1. Extract Archive 0 (animations)
2. Study animation ID 808 (walk)
3. Identify:
   - Frame count
   - Bone transformations per frame
   - Timing data
4. Create AnimationParser.js
```

**Day 5: Integration**
```
1. Wire CacheReader to load real models
2. Test with 5-10 different model IDs
3. Verify geometry matches OSRS appearance
4. Replace placeholder models in PlayerModel.js
```

**Success Criteria:**
- ✅ Load at least 10 different models from cache
- ✅ Models look correct (not garbled)
- ✅ Can identify body part models (head, torso, etc.)
- ✅ Understand animation data structure

---

### 2. Three.js Skeletal Animation - HIGH PRIORITY

**Time Investment:** 4-6 hours  
**Why Critical:** Needed for character animations

#### What to Study

**Core Concepts:**
1. **Bones and Skeletons**
   - THREE.Bone class
   - Bone hierarchy (parent/child)
   - Bone transformations (position, rotation, scale)

2. **Skinned Meshes**
   - THREE.SkinnedMesh
   - Skin indices (which bones affect which vertices)
   - Skin weights (how much each bone affects vertex)

3. **Animation System**
   - THREE.AnimationMixer (plays animations)
   - THREE.AnimationClip (animation data)
   - THREE.KeyframeTrack (bone transforms over time)

4. **Animation Blending**
   - Crossfade between animations
   - Blend walk → run → idle smoothly
   - Weight-based blending

#### Learning Resources

**Official Documentation:**
- Three.js Docs: SkinnedMesh
  - https://threejs.org/docs/#api/en/objects/SkinnedMesh
- Three.js Docs: AnimationMixer
  - https://threejs.org/docs/#api/en/animation/AnimationMixer

**Examples to Study:**
- Three.js Examples: webgl_animation_skinning_blending
  - Shows character animation with blending
  - Source code is well-commented
  
- Three.js Examples: webgl_animation_keyframes
  - Shows keyframe animation creation
  - Good reference for custom animations

**Video Tutorials:**
- "Three.js Character Animation" by SimonDev (YouTube)
  - 30-minute tutorial
  - Covers bones, skinning, and animation
  
- "Three.js Animation System" by Bruno Simon
  - Part of Three.js Journey course
  - Detailed explanation of AnimationMixer

#### Action Steps

**Day 1: Basic Skeleton**
```javascript
// Create simple 2-bone skeleton (leg)
const bone1 = new THREE.Bone(); // hip
const bone2 = new THREE.Bone(); // knee
bone1.add(bone2);
bone2.position.y = -1;

// Create skeleton
const skeleton = new THREE.Skeleton([bone1, bone2]);

// Test manual animation
bone2.rotation.x = Math.sin(time) * 0.5;
```

**Day 2: RuneScape Skeleton**
```javascript
// Map RS bone IDs to Three.js bones
const rsBoneMap = {
  0: 'root',
  1: 'torso',
  2: 'head',
  3: 'arm_left',
  4: 'arm_right',
  5: 'leg_left',
  6: 'leg_right'
};

// Create skeleton matching RS hierarchy
// Load bone positions from Archive 1
```

**Day 3: Animation Loading**
```javascript
// Parse RS animation data
const animData = parseRSAnimation(808); // Walk animation

// Convert to Three.js AnimationClip
const clip = new THREE.AnimationClip('walk', 2.0, [
  new THREE.KeyframeTrack(
    'bone1.rotation[x]',
    [0, 0.5, 1.0],
    [0, 0.3, 0]
  ),
  // ... more tracks
]);

// Play animation
const mixer = new THREE.AnimationMixer(playerMesh);
mixer.clipAction(clip).play();
```

**Day 4: Integration with PlayerModel**
```javascript
// Update PlayerModel.js
class PlayerModel {
  async loadBodyParts() {
    // Load models with bone weights
    const torsoData = await this.cacheReader.getModel(18);
    
    // Create SkinnedMesh instead of Mesh
    const mesh = new THREE.SkinnedMesh(geometry, material);
    mesh.add(this.skeleton);
    mesh.bind(this.skeleton);
  }
  
  update(delta) {
    // Update animation
    this.mixer.update(delta);
  }
}
```

**Success Criteria:**
- ✅ Understand Bone/Skeleton/SkinnedMesh relationship
- ✅ Create custom animations programmatically
- ✅ Load RS animation data into Three.js format
- ✅ Smooth animation blending works

---

### 3. Raycasting & Click Interactions - MEDIUM PRIORITY

**Time Investment:** 3-4 hours  
**Why Important:** Core gameplay mechanic

#### What to Study

**Raycasting Basics:**
- Mouse → 3D world coordinate conversion
- Ray-object intersection testing
- Finding clicked objects

**Three.js Raycaster API:**
```javascript
const raycaster = new THREE.Raycaster();
const mouse = new THREE.Vector2();

// Convert mouse to NDC
mouse.x = (event.clientX / window.innerWidth) * 2 - 1;
mouse.y = -(event.clientY / window.innerHeight) * 2 + 1;

// Cast ray
raycaster.setFromCamera(mouse, camera);
const intersects = raycaster.intersectObjects(scene.children, true);
```

#### Learning Resources

**Official Examples:**
- Three.js Example: webgl_interactive_cubes
  - Basic click detection
  - Hover effects

- Three.js Example: webgl_interactive_raycasting_points
  - Point cloud intersection
  - Shows performance optimization

**Tutorials:**
- "Mouse Picking in Three.js" (Three.js Fundamentals)
  - Clear explanation of coordinate systems
  - Multiple picking techniques

#### Action Steps

**Hour 1: Ground Clicking (Click-to-Walk)**
```javascript
// Add to setupControls() in main.js
canvas.addEventListener('click', (event) => {
  const rect = canvas.getBoundingClientRect();
  mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
  mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
  
  raycaster.setFromCamera(mouse, camera);
  const intersects = raycaster.intersectObject(terrain);
  
  if (intersects.length > 0) {
    const point = intersects[0].point;
    gameState.player.moveTo(point.x, point.y, point.z);
  }
});
```

**Hour 2: NPC Clicking**
```javascript
// Make NPCs clickable
const npcMeshes = [];
gameState.npcs.forEach(npc => {
  npcMeshes.push(npc.getGroup());
});

raycaster.setFromCamera(mouse, camera);
const intersects = raycaster.intersectObjects(npcMeshes, true);

if (intersects.length > 0) {
  const clickedNPC = findNPCFromMesh(intersects[0].object);
  showContextMenu(clickedNPC.getActions());
}
```

**Hour 3: Right-Click Menu**
```javascript
// Create context menu UI
function showContextMenu(actions, x, y) {
  const menu = document.createElement('div');
  menu.className = 'context-menu';
  menu.style.left = x + 'px';
  menu.style.top = y + 'px';
  
  actions.forEach(action => {
    const item = document.createElement('div');
    item.textContent = action.text;
    item.style.color = action.color;
    item.onclick = () => handleAction(action);
    menu.appendChild(item);
  });
  
  document.body.appendChild(menu);
}
```

**Success Criteria:**
- ✅ Click on ground to move player
- ✅ Click on NPC to select
- ✅ Right-click shows context menu
- ✅ Menu actions trigger correctly

---

## 📚 Secondary Research (Next 2 Weeks)

### 4. Terrain Rendering from Height Maps

**Time Investment:** 4-5 hours

**Key Topics:**
- Height map to mesh conversion
- Texture splatting (multiple ground textures)
- Normal map generation
- Collision mesh creation

**Resources:**
- Three.js terrain examples
- "Procedural Terrain Generation" articles
- RuneScape map format documentation

**Action Steps:**
1. Parse map region from Archive 5
2. Extract height data (128x128 grid)
3. Generate PlaneGeometry with heights
4. Apply ground textures
5. Add to scene with collision

---

### 5. Network Synchronization

**Time Investment:** 5-6 hours

**Key Topics:**
- Client-side prediction
- Server reconciliation
- Entity interpolation
- Dead reckoning

**Resources:**
- Gabriel Gambetta: "Fast-Paced Multiplayer"
  - Part I: Client-Server Game Architecture
  - Part II: Client-Side Prediction
  - Part III: Server Reconciliation
  - Part IV: Lag Compensation
  
- Valve: Source Multiplayer Networking
  - Authoritative server
  - Lag compensation techniques

**Action Steps:**
1. Implement movement prediction
2. Add server position reconciliation
3. Smooth interpolation between states
4. Test with artificial lag

---

### 6. Performance Optimization

**Time Investment:** 3-4 hours

**Key Topics:**
- Chrome DevTools Performance profiling
- Three.js Stats.js addon
- Draw call reduction
- Frustum culling
- LOD (Level of Detail)

**Resources:**
- "Optimizing Three.js Performance" (Bruno Simon)
- Chrome DevTools documentation
- GPU profiling tools

**Action Steps:**
1. Profile current performance
2. Identify bottlenecks
3. Implement instanced rendering for NPCs
4. Add frustum culling
5. Benchmark improvements

---

## 🛠️ Practical Exercises

### Week 1 Exercises

**Monday: Cache Model Loading**
```
Goal: Load 10 different models from cache
Time: 2-3 hours
Output: Updated CacheReader that loads real models
```

**Tuesday: Animation Parsing**
```
Goal: Parse walk animation (ID 808)
Time: 2-3 hours
Output: AnimationParser.js with working parse() function
```

**Wednesday: Three.js Skeleton**
```
Goal: Create RS skeleton in Three.js
Time: 2-3 hours
Output: Working skeleton with 10+ bones
```

**Thursday: Animation Playback**
```
Goal: Play walk animation on player
Time: 2-3 hours
Output: Walking player with smooth animation
```

**Friday: Click Detection**
```
Goal: Implement click-to-walk
Time: 2-3 hours
Output: Player moves to clicked position
```

### Week 2 Exercises

**Monday: Terrain Loading**
```
Goal: Load one map region
Time: 3-4 hours
Output: Terrain mesh from cache data
```

**Tuesday: NPC Interaction**
```
Goal: Right-click menu on NPCs
Time: 2-3 hours
Output: Working context menu
```

**Wednesday: Equipment Models**
```
Goal: Load real equipment from cache
Time: 2-3 hours
Output: Player wearing actual armor
```

**Thursday: Performance**
```
Goal: Profile and optimize
Time: 3-4 hours
Output: 60 FPS with 20+ NPCs
```

**Friday: Integration Testing**
```
Goal: Test all new features together
Time: 2-3 hours
Output: Polished Phase 3 demo
```

---

## 📖 Recommended Reading Order

### This Week
1. ✅ **Cache format documentation** (OpenRS2)
2. ✅ **Model parsing code** (RSCacheEditor source)
3. ✅ **Three.js animation docs** (official)
4. ✅ **Raycasting tutorial** (Three.js Fundamentals)

### Next Week
5. **Terrain generation** (Three.js examples)
6. **Network synchronization** (Gambetta articles)
7. **Performance optimization** (Bruno Simon)

### Month 2
8. **ECS architecture** (Game Programming Patterns)
9. **Database design** (MMO data schemas)
10. **Security** (Server authority pattern)

---

## 🎓 Study Tips

### Effective Learning Strategies

**1. Code Along**
- Don't just read, implement immediately
- Start with simple examples
- Gradually add complexity

**2. Take Notes**
- Document key findings
- Keep code snippets
- Screenshot important diagrams

**3. Build Incrementally**
- Don't try to understand everything at once
- Get one thing working, then move on
- Iterate and improve

**4. Use Debugger**
- Step through cache reading code
- Inspect animation data structures
- Visualize bone hierarchies

**5. Ask Questions**
- RuneScape emulation Discord
- Three.js Discourse
- Stack Overflow

---

## 🎯 Success Metrics

### By End of Week 1
- ✅ Understand cache archive structure
- ✅ Load 10+ models from cache
- ✅ Parse one animation
- ✅ Create Three.js skeleton

### By End of Week 2
- ✅ Play animations on player
- ✅ Load terrain from cache
- ✅ Click-to-walk working
- ✅ Right-click menus functional

### By End of Month
- ✅ Complete Phase 3 (Enhanced Rendering)
- ✅ All real models loading
- ✅ Smooth animations
- ✅ Full interaction system

---

## 🔗 Quick Reference Links

### Documentation
- Three.js Docs: https://threejs.org/docs/
- MDN Web Docs: https://developer.mozilla.org/
- Rust Docs: https://doc.rust-lang.org/
- OpenRS2: https://archive.openrs2.org/

### Tools
- HxD Hex Editor: https://mh-nexus.de/en/hxd/
- Chrome DevTools: Built into Chrome
- Three.js Inspector: Browser extension

### Communities
- Three.js Discourse: https://discourse.threejs.org/
- r/rust: https://reddit.com/r/rust
- OSRS Wiki: https://oldschool.runescape.wiki/

---

## 💪 Motivation

You've already built:
- ✅ Working Rust server
- ✅ Cache integration system
- ✅ 3D rendering engine
- ✅ Entity system with AI

**You're 50% done with Phase 3!**

The research above will unlock:
- Real RuneScape models in your game
- Smooth character animations
- Interactive gameplay
- Polished user experience

**Time Investment:** ~20 hours of focused research
**Payoff:** Complete, playable game foundation

**You got this! 🚀**

---

*Focus on cache format first - it unlocks everything else!*