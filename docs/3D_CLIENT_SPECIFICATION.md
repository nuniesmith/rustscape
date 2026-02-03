# Rustscape 3D Client Technical Specification

**Version:** 1.0  
**Date:** 2026-02-03  
**Target:** WebGL-based 3D RuneScape client using Build 560 cache data

---

## Executive Summary

This specification outlines the architecture for a **3D isometric WebGL client** that matches the authentic RuneScape 2006-2009 aesthetic shown in the reference screenshots. The client will use Three.js for 3D rendering and integrate with the existing Build 560 cache data (74.5 MiB, 29 archives) to load models, textures, and sprites.

**Key Goals:**
1. Authentic RuneScape 2D3 (2.5D isometric) visual style
2. Load and render cache data (models, textures, sprites)
3. 60 FPS performance on mid-range hardware
4. Maintain current server protocol compatibility
5. Progressive enhancement (fallback to 2D if WebGL unavailable)

---

## Architecture Overview

### Technology Stack

```
┌─────────────────────────────────────────────────┐
│              Browser (Client)                   │
├─────────────────────────────────────────────────┤
│  Presentation Layer                             │
│  - HTML5 Canvas (WebGL context)                 │
│  - CSS3 (UI overlays)                           │
├─────────────────────────────────────────────────┤
│  Rendering Engine                               │
│  - Three.js r150+ (WebGL abstraction)           │
│  - Custom shaders (GLSL)                        │
│  - Isometric camera controller                  │
├─────────────────────────────────────────────────┤
│  Game Logic Layer                               │
│  - Scene manager                                │
│  - Entity system (players, NPCs, objects)       │
│  - Animation controller                         │
│  - Collision detection                          │
├─────────────────────────────────────────────────┤
│  Cache System                                   │
│  - Cache loader (OSRS cache format)             │
│  - Model parser (.dat format)                   │
│  - Texture decoder (sprites, textures)          │
│  - Archive extractor                            │
├─────────────────────────────────────────────────┤
│  Network Layer                                  │
│  - WebSocket (existing protocol)                │
│  - Packet handlers                              │
│  - State synchronization                        │
├─────────────────────────────────────────────────┤
│  Asset Management                               │
│  - Lazy loading                                 │
│  - LOD system                                   │
│  - Memory management                            │
└─────────────────────────────────────────────────┘
                      ↕
┌─────────────────────────────────────────────────┐
│         Rustscape Server (Existing)             │
│         - Axum + WebSocket                      │
│         - JSON packets                          │
└─────────────────────────────────────────────────┘
```

---

## Cache Integration

### Build 560 Cache Structure

Your cache contains:
- **29 archives** (idx files + main data file)
- **73,880 groups** (models, textures, sprites, configs)
- **74.5 MiB total**

#### Archive Types (Standard OSRS Cache)
```
Archive 0:  Animations
Archive 1:  Skeletons
Archive 2:  Configs (items, NPCs, objects)
Archive 3:  Interfaces
Archive 4:  Sound effects
Archive 5:  Maps (landscape data)
Archive 6:  Music
Archive 7:  Models (3D geometry)
Archive 8:  Sprites (2D images)
Archive 9:  Textures
Archive 10: Binary data
Archive 11: Jingles/fanfares
Archive 12: Client scripts
Archive 13: Fonts
Archive 14-28: Varies by build
```

### Cache Loading Strategy

#### Phase 1: Essential Assets (Load on Startup)
```javascript
const ESSENTIAL_ASSETS = {
    models: [
        1,    // Player male model
        2,    // Player female model
        100,  // Basic terrain tile
        101,  // Wall
        // ... common models
    ],
    textures: [
        0,    // Grass texture
        1,    // Stone texture
        // ... base textures
    ],
    sprites: [
        0,    // UI backgrounds
        1,    // Skill icons
        // ... UI elements
    ]
};
```

#### Phase 2: On-Demand Loading
- Load models/textures when entering new regions
- Unload assets outside view distance
- Cache frequently used assets in memory

### Cache Parser Implementation

```javascript
// Simplified cache structure
class CacheReader {
    constructor(cacheUrl) {
        this.baseUrl = cacheUrl; // '/assets/data_caches/560/'
        this.archives = new Map();
        this.loaded = new Set();
    }
    
    async loadArchive(archiveId) {
        if (this.archives.has(archiveId)) {
            return this.archives.get(archiveId);
        }
        
        const idx = await fetch(`${this.baseUrl}/main_file_cache.idx${archiveId}`);
        const idxBuffer = await idx.arrayBuffer();
        
        const dat = await fetch(`${this.baseUrl}/main_file_cache.dat2`);
        const datBuffer = await dat.arrayBuffer();
        
        const archive = new Archive(idxBuffer, datBuffer);
        this.archives.set(archiveId, archive);
        return archive;
    }
    
    async getModel(modelId) {
        const archive = await this.loadArchive(7); // Models in archive 7
        const modelData = archive.getGroup(modelId);
        return this.parseModel(modelData);
    }
    
    parseModel(data) {
        // Parse OSRS model format (.dat)
        // Returns: { vertices, faces, textures, colors }
        const view = new DataView(data);
        let offset = 0;
        
        // Model header
        const vertexCount = view.getUint16(offset); offset += 2;
        const faceCount = view.getUint16(offset); offset += 2;
        const texturedFaceCount = view.getUint8(offset); offset += 1;
        
        // Vertices (x, y, z coordinates)
        const vertices = [];
        for (let i = 0; i < vertexCount; i++) {
            vertices.push({
                x: view.getInt16(offset), offset += 2,
                y: view.getInt16(offset), offset += 2,
                z: view.getInt16(offset), offset += 2
            });
        }
        
        // Faces (triangle indices)
        const faces = [];
        for (let i = 0; i < faceCount; i++) {
            faces.push({
                a: view.getUint16(offset), offset += 2,
                b: view.getUint16(offset), offset += 2,
                c: view.getUint16(offset), offset += 2
            });
        }
        
        // Face colors
        const colors = [];
        for (let i = 0; i < faceCount; i++) {
            colors.push(view.getUint16(offset)); offset += 2;
        }
        
        return { vertices, faces, colors, texturedFaceCount };
    }
    
    async getTexture(textureId) {
        const archive = await this.loadArchive(9); // Textures in archive 9
        const textureData = archive.getGroup(textureId);
        return this.parseTexture(textureData);
    }
    
    async getSprite(spriteId) {
        const archive = await this.loadArchive(8); // Sprites in archive 8
        const spriteData = archive.getGroup(spriteId);
        return this.parseSprite(spriteData);
    }
}
```

---

## 3D Rendering System

### Three.js Scene Setup

```javascript
import * as THREE from 'three';

class RustscapeRenderer {
    constructor(canvas) {
        this.canvas = canvas;
        this.scene = new THREE.Scene();
        this.camera = this.createIsometricCamera();
        this.renderer = new THREE.WebGLRenderer({
            canvas: this.canvas,
            antialias: true,
            alpha: false,
            powerPreference: 'high-performance'
        });
        
        this.renderer.setPixelRatio(window.devicePixelRatio);
        this.renderer.setSize(window.innerWidth * 0.7, window.innerHeight * 0.8);
        this.renderer.shadowMap.enabled = true;
        this.renderer.shadowMap.type = THREE.PCFSoftShadowMap;
        
        this.setupLighting();
        this.setupFog();
    }
    
    createIsometricCamera() {
        // RuneScape uses isometric projection at ~26.565° (arctan(0.5))
        const aspect = this.canvas.width / this.canvas.height;
        const frustumSize = 20;
        
        const camera = new THREE.OrthographicCamera(
            frustumSize * aspect / -2,  // left
            frustumSize * aspect / 2,   // right
            frustumSize / 2,            // top
            frustumSize / -2,           // bottom
            0.1,                        // near
            1000                        // far
        );
        
        // Position camera for isometric view
        camera.position.set(15, 15, 15);
        camera.lookAt(0, 0, 0);
        
        return camera;
    }
    
    setupLighting() {
        // Ambient light (fills shadows)
        const ambient = new THREE.AmbientLight(0x666666);
        this.scene.add(ambient);
        
        // Directional light (sun) - classic RS lighting
        const sun = new THREE.DirectionalLight(0xffffee, 0.8);
        sun.position.set(50, 100, 50);
        sun.castShadow = true;
        sun.shadow.mapSize.width = 2048;
        sun.shadow.mapSize.height = 2048;
        sun.shadow.camera.near = 0.5;
        sun.shadow.camera.far = 500;
        this.scene.add(sun);
    }
    
    setupFog() {
        // Distance fog like RuneScape
        this.scene.fog = new THREE.Fog(0x9eb4b8, 50, 100);
    }
    
    render() {
        this.renderer.render(this.scene, this.camera);
    }
}
```

### Model Rendering

```javascript
class ModelRenderer {
    constructor(scene) {
        this.scene = scene;
        this.modelCache = new Map();
        this.cacheReader = new CacheReader('/assets/data_caches/560');
    }
    
    async loadModel(modelId) {
        if (this.modelCache.has(modelId)) {
            return this.modelCache.get(modelId).clone();
        }
        
        const modelData = await this.cacheReader.getModel(modelId);
        const mesh = this.createMeshFromModelData(modelData);
        
        this.modelCache.set(modelId, mesh);
        return mesh.clone();
    }
    
    createMeshFromModelData(modelData) {
        const geometry = new THREE.BufferGeometry();
        
        // Convert vertices to Three.js format
        const positions = [];
        const colors = [];
        const indices = [];
        
        modelData.vertices.forEach(v => {
            positions.push(v.x / 128, v.y / 128, v.z / 128); // Scale to world units
        });
        
        modelData.faces.forEach((face, i) => {
            indices.push(face.a, face.b, face.c);
            
            // Convert OSRS color (RGB565) to RGB
            const color = this.convertColor(modelData.colors[i]);
            for (let j = 0; j < 3; j++) {
                colors.push(color.r, color.g, color.b);
            }
        });
        
        geometry.setAttribute('position', new THREE.Float32BufferAttribute(positions, 3));
        geometry.setAttribute('color', new THREE.Float32BufferAttribute(colors, 3));
        geometry.setIndex(indices);
        geometry.computeVertexNormals();
        
        // Use vertex colors (classic RS style)
        const material = new THREE.MeshLambertMaterial({
            vertexColors: true,
            flatShading: true // Low-poly RS look
        });
        
        return new THREE.Mesh(geometry, material);
    }
    
    convertColor(rgb565) {
        // Convert RGB565 to RGB888
        const r = ((rgb565 >> 11) & 0x1f) * 255 / 31;
        const g = ((rgb565 >> 5) & 0x3f) * 255 / 63;
        const b = (rgb565 & 0x1f) * 255 / 31;
        return { r: r / 255, g: g / 255, b: b / 255 };
    }
}
```

### Terrain System

```javascript
class TerrainRenderer {
    constructor(scene, cacheReader) {
        this.scene = scene;
        this.cache = cacheReader;
        this.chunks = new Map();
        this.CHUNK_SIZE = 64; // 64x64 tiles per chunk
    }
    
    async loadRegion(regionX, regionY) {
        const chunkKey = `${regionX}_${regionY}`;
        if (this.chunks.has(chunkKey)) {
            return this.chunks.get(chunkKey);
        }
        
        // Load map data from cache (archive 5)
        const mapArchive = await this.cache.loadArchive(5);
        const mapData = mapArchive.getGroup(regionX * 256 + regionY);
        
        const chunk = this.createTerrainChunk(mapData);
        this.scene.add(chunk);
        this.chunks.set(chunkKey, chunk);
        
        return chunk;
    }
    
    createTerrainChunk(mapData) {
        const group = new THREE.Group();
        const view = new DataView(mapData);
        let offset = 0;
        
        // Parse terrain heights
        const heights = [];
        for (let x = 0; x < this.CHUNK_SIZE; x++) {
            heights[x] = [];
            for (let y = 0; y < this.CHUNK_SIZE; y++) {
                heights[x][y] = view.getInt8(offset++);
            }
        }
        
        // Create terrain mesh
        const geometry = new THREE.PlaneGeometry(
            this.CHUNK_SIZE,
            this.CHUNK_SIZE,
            this.CHUNK_SIZE - 1,
            this.CHUNK_SIZE - 1
        );
        
        // Apply height data
        const positions = geometry.attributes.position.array;
        for (let i = 0; i < positions.length; i += 3) {
            const x = Math.floor((i / 3) % this.CHUNK_SIZE);
            const y = Math.floor((i / 3) / this.CHUNK_SIZE);
            positions[i + 2] = heights[x][y] / 4; // Z is height
        }
        
        geometry.computeVertexNormals();
        
        // Grass texture
        const material = new THREE.MeshLambertMaterial({
            color: 0x3a6b1f, // Grass green
            flatShading: true
        });
        
        const terrain = new THREE.Mesh(geometry, material);
        terrain.rotation.x = -Math.PI / 2; // Lay flat
        terrain.receiveShadow = true;
        
        group.add(terrain);
        return group;
    }
}
```

---

## Entity System

### Player Rendering

```javascript
class Player extends THREE.Group {
    constructor(id, username, modelRenderer) {
        super();
        
        this.id = id;
        this.username = username;
        this.modelRenderer = modelRenderer;
        
        this.position.set(0, 0, 0);
        this.targetPosition = null;
        this.moveSpeed = 2.0; // tiles per second
        
        this.loadModel();
        this.createNameplate();
    }
    
    async loadModel() {
        // Load base player model (ID 1 for male, 2 for female)
        const bodyModel = await this.modelRenderer.loadModel(1);
        this.add(bodyModel);
        
        // Equipment models loaded separately and attached
        this.equipment = {
            head: null,
            cape: null,
            weapon: null,
            body: null,
            legs: null,
            // ... etc
        };
    }
    
    createNameplate() {
        // Create canvas for text
        const canvas = document.createElement('canvas');
        canvas.width = 256;
        canvas.height = 64;
        const ctx = canvas.getContext('2d');
        
        ctx.font = 'bold 24px RuneScape';
        ctx.fillStyle = '#ffffff';
        ctx.strokeStyle = '#000000';
        ctx.lineWidth = 3;
        ctx.textAlign = 'center';
        
        ctx.strokeText(this.username, 128, 40);
        ctx.fillText(this.username, 128, 40);
        
        const texture = new THREE.CanvasTexture(canvas);
        const material = new THREE.SpriteMaterial({ map: texture });
        const sprite = new THREE.Sprite(material);
        sprite.scale.set(2, 0.5, 1);
        sprite.position.y = 2; // Above player
        
        this.add(sprite);
    }
    
    async equipItem(slot, itemId) {
        // Remove old equipment
        if (this.equipment[slot]) {
            this.remove(this.equipment[slot]);
        }
        
        // Load new equipment model
        const model = await this.modelRenderer.loadModel(itemId);
        this.equipment[slot] = model;
        this.add(model);
    }
    
    moveTo(x, y, z) {
        this.targetPosition = new THREE.Vector3(x, z, y); // Note: Y and Z swapped
    }
    
    update(deltaTime) {
        if (this.targetPosition) {
            const direction = this.targetPosition.clone().sub(this.position);
            const distance = direction.length();
            
            if (distance < 0.1) {
                this.position.copy(this.targetPosition);
                this.targetPosition = null;
            } else {
                direction.normalize();
                const moveDistance = this.moveSpeed * deltaTime;
                this.position.add(direction.multiplyScalar(Math.min(moveDistance, distance)));
            }
        }
    }
}
```

---

## UI Overlay System

The 3D canvas will be the main view, with HTML/CSS overlays for UI panels (exactly like RuneLite).

```html
<!-- game.html -->
<div id="game-container">
    <!-- 3D WebGL Canvas (left side, 70% width) -->
    <canvas id="game-canvas"></canvas>
    
    <!-- UI Overlays -->
    <div id="minimap-container">
        <canvas id="minimap-canvas"></canvas>
        <div id="compass"></div>
        <div id="xp-orbs">
            <div class="xp-orb" id="hp-orb"></div>
            <div class="xp-orb" id="prayer-orb"></div>
            <div class="xp-orb" id="run-orb"></div>
        </div>
    </div>
    
    <!-- Right sidebar (tabs) -->
    <div id="sidebar">
        <div id="tab-buttons">
            <button class="tab-btn" data-tab="combat">⚔️</button>
            <button class="tab-btn" data-tab="stats">📊</button>
            <button class="tab-btn" data-tab="quest">📜</button>
            <button class="tab-btn" data-tab="inventory">🎒</button>
            <button class="tab-btn" data-tab="equipment">👕</button>
            <button class="tab-btn" data-tab="prayer">🙏</button>
            <button class="tab-btn" data-tab="spellbook">✨</button>
        </div>
        <div id="tab-content">
            <!-- Tab panels here -->
        </div>
    </div>
    
    <!-- Chat box (bottom) -->
    <div id="chat-box">
        <div id="chat-messages"></div>
        <input id="chat-input" type="text" />
    </div>
</div>
```

```css
/* Classic RuneScape brown/stone theme */
:root {
    --rs-brown-dark: #3e3529;
    --rs-brown-med: #524a3d;
    --rs-brown-light: #6b614e;
    --rs-stone: #4e4a40;
    --rs-gold: #d4a017;
    --rs-text: #ffff00;
}

#game-canvas {
    position: absolute;
    left: 0;
    top: 0;
    width: 70%;
    height: calc(100% - 150px);
    background: #000;
}

#sidebar {
    position: absolute;
    right: 0;
    top: 0;
    width: 30%;
    height: calc(100% - 150px);
    background: var(--rs-brown-dark);
    border-left: 2px solid var(--rs-gold);
}

#chat-box {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    height: 150px;
    background: var(--rs-brown-med);
    border-top: 2px solid var(--rs-gold);
}
```

---

## Performance Optimization

### Level of Detail (LOD) System

```javascript
class LODManager {
    constructor(camera) {
        this.camera = camera;
        this.levels = [
            { distance: 10, quality: 'high' },
            { distance: 30, quality: 'medium' },
            { distance: 50, quality: 'low' },
            { distance: 100, quality: 'billboard' }
        ];
    }
    
    updateEntityLOD(entity) {
        const distance = this.camera.position.distanceTo(entity.position);
        
        for (const level of this.levels) {
            if (distance < level.distance) {
                entity.setQuality(level.quality);
                return;
            }
        }
        
        entity.visible = false; // Too far, don't render
    }
}
```

### Frustum Culling

```javascript
class CullingManager {
    constructor(camera) {
        this.camera = camera;
        this.frustum = new THREE.Frustum();
        this.projectionMatrix = new THREE.Matrix4();
    }
    
    update() {
        this.projectionMatrix.multiplyMatrices(
            this.camera.projectionMatrix,
            this.camera.matrixWorldInverse
        );
        this.frustum.setFromProjectionMatrix(this.projectionMatrix);
    }
    
    isVisible(object) {
        return this.frustum.intersectsObject(object);
    }
}
```

### Chunk Loading

```javascript
class ChunkManager {
    constructor(terrainRenderer, playerPosition) {
        this.terrain = terrainRenderer;
        this.playerPos = playerPosition;
        this.LOAD_RADIUS = 2; // Load 2 chunks in each direction
        this.loadedChunks = new Set();
    }
    
    update() {
        const playerChunkX = Math.floor(this.playerPos.x / 64);
        const playerChunkY = Math.floor(this.playerPos.y / 64);
        
        // Load nearby chunks
        for (let dx = -this.LOAD_RADIUS; dx <= this.LOAD_RADIUS; dx++) {
            for (let dy = -this.LOAD_RADIUS; dy <= this.LOAD_RADIUS; dy++) {
                const chunkX = playerChunkX + dx;
                const chunkY = playerChunkY + dy;
                const key = `${chunkX}_${chunkY}`;
                
                if (!this.loadedChunks.has(key)) {
                    this.terrain.loadRegion(chunkX, chunkY);
                    this.loadedChunks.add(key);
                }
            }
        }
        
        // Unload distant chunks
        this.loadedChunks.forEach(key => {
            const [x, y] = key.split('_').map(Number);
            const distance = Math.max(Math.abs(x - playerChunkX), Math.abs(y - playerChunkY));
            
            if (distance > this.LOAD_RADIUS + 1) {
                this.terrain.unloadRegion(x, y);
                this.loadedChunks.delete(key);
            }
        });
    }
}
```

---

## Camera Controls

```javascript
class CameraController {
    constructor(camera, canvas) {
        this.camera = camera;
        this.canvas = canvas;
        
        // Camera settings
        this.rotationAngle = 0; // 0, 90, 180, 270 degrees
        this.pitch = 26.565; // Isometric angle
        this.zoom = 1.0;
        this.targetPosition = new THREE.Vector3(0, 0, 0);
        
        this.setupControls();
    }
    
    setupControls() {
        // Mouse wheel for zoom
        this.canvas.addEventListener('wheel', (e) => {
            e.preventDefault();
            this.zoom += e.deltaY * -0.001;
            this.zoom = Math.max(0.5, Math.min(2.0, this.zoom));
            this.updateCamera();
        });
        
        // Arrow keys or middle mouse drag for rotation
        window.addEventListener('keydown', (e) => {
            if (e.key === 'ArrowLeft') {
                this.rotationAngle = (this.rotationAngle + 90) % 360;
                this.updateCamera();
            } else if (e.key === 'ArrowRight') {
                this.rotationAngle = (this.rotationAngle - 90) % 360;
                this.updateCamera();
            }
        });
    }
    
    followPlayer(playerPosition) {
        this.targetPosition.copy(playerPosition);
        this.updateCamera();
    }
    
    updateCamera() {
        const distance = 20 / this.zoom;
        const angleRad = this.rotationAngle * Math.PI / 180;
        const pitchRad = this.pitch * Math.PI / 180;
        
        this.camera.position.x = this.targetPosition.x + distance * Math.cos(angleRad) * Math.cos(pitchRad);
        this.camera.position.y = this.targetPosition.y + distance * Math.sin(pitchRad);
        this.camera.position.z = this.targetPosition.z + distance * Math.sin(angleRad) * Math.cos(pitchRad);
        
        this.camera.lookAt(this.targetPosition);
    }
}
```

---

## Animation System

```javascript
class AnimationController {
    constructor(cacheReader) {
        this.cache = cacheReader;
        this.animations = new Map();
        this.activeAnimations = new Map();
    }
    
    async loadAnimation(animId) {
        if (this.animations.has(animId)) {
            return this.animations.get(animId);
        }
        
        const archive = await this.cache.loadArchive(0); // Animations
        const animData = archive.getGroup(animId);
        const animation = this.parseAnimation(animData);
        
        this.animations.set(animId, animation);
        return animation;
    }
    
    parseAnimation(data) {
        // Parse OSRS animation format
        // Returns frame data, transformations, durations
        const view = new DataView(data);
        let offset = 0;
        
        const frameCount = view.getUint16(offset); offset += 2;
        const frames = [];
        
        for (let i = 0; i < frameCount; i++) {
            frames.push({
                duration: view.getUint16(offset), offset += 2,
                // ... bone transformations
            });
        }
        
        return { frameCount, frames };
    }
    
    playAnimation(entity, animId, loop = false) {
        this.activeAnimations.set(entity.id, {
            animation: animId,
            frame: 0,
            time: 0,
            loop
        });
    }
    
    update(deltaTime) {
        this.activeAnimations.forEach((state, entityId) => {
            state.time += deltaTime;
            const animation = this.animations.get(state.animation);
            
            if (!animation) return;
            
            // Update frame based on time
            const currentFrame = animation.frames[state.frame];
            if (state.time >= currentFrame.duration) {
                state.frame++;
                state.time = 0;
                
                if (state.frame >= animation.frameCount) {
                    if (state.loop) {
                        state.frame = 0;
                    } else {
                        this.activeAnimations.delete(entityId);
                    }
                }
            }
        });
    }
}
```

---

## Implementation Roadmap

### Phase 1: Foundation (Week 1-2)
- [ ] Set up Three.js project structure
- [ ] Implement cache reader (basic)
- [ ] Create isometric camera
- [ ] Render simple terrain plane
- [ ] Display one player model
- [ ] Integrate with existing WebSocket

### Phase 2: Cache Integration (Week 3-4)
- [ ] Full model parser (.dat format)
- [ ] Texture decoder
- [ ] Sprite loader
- [ ] Equipment rendering
- [ ] Terrain from cache data
- [ ] NPC models

### Phase 3: UI & Polish (Week 5-6)
- [ ] Classic RS UI theme
- [ ] Minimap (3D → 2D projection)
- [ ] Tab interface (inventory, stats, etc.)
- [ ] Chat box integration
- [ ] XP orbs
- [ ] Right-click menus

### Phase 4: Performance (Week 7-8)
- [ ] LOD system
- [ ] Frustum culling
- [ ] Chunk loading/unloading
- [ ] Sprite batching
- [ ] Memory optimization
- [ ] 60 FPS target achieved

### Phase 5: Features (Week 9-10)
- [ ] Animations (walk, attack, idle)
- [ ] Camera rotation
- [ ] Zoom controls
- [ ] Click-to-walk in 3D
- [ ] Equipment system
- [ ] Combat visuals

---

## Performance Targets

- **Frame Rate**: 60 FPS minimum
- **Load Time**: < 5 seconds initial load
- **Memory**: < 500 MB RAM usage
- **Draw Calls**: < 500 per frame
- **Entities**: Support 100+ visible entities

### Optimization Checklist
- ✅ Frustum culling (don't render off-screen)
- ✅ LOD system (reduce detail at distance)
- ✅ Chunk-based loading (lazy load terrain)
- ✅ Geometry instancing (reuse models)
- ✅ Texture atlases (reduce texture swaps)
- ✅ Object pooling (reuse Three.js objects)
- ✅ Worker threads (cache parsing off main thread)

---

## Fallback Strategy

For browsers without WebGL or low-end devices:

```javascript
if (!WebGLDetector.isAvailable()) {
    // Fall back to 2D canvas client
    window.location.href = '/game-2d.html';
}
```

Keep 2D client as fallback option.

---

## Dependencies

```json
{
  "dependencies": {
    "three": "^0.150.0",
    "stats.js": "^0.17.0"
  },
  "devDependencies": {
    "vite": "^4.0.0"
  }
}
```

---

## File Structure

```
client/dist/
├── game-3d.html              # 3D client entry
├── game-2d.html              # 2D fallback
├── css/
│   ├── rs-theme.css          # Classic RS styling
│   └── ui-panels.css
├── js/
│   ├── main-3d.js            # 3D client entry
│   ├── cache/
│   │   ├── CacheReader.js
│   │   ├── ModelParser.js
│   │   ├── TextureDecoder.js
│   │   └── Archive.js
│   ├── rendering/
│   │   ├── RustscapeRenderer.js
│   │   ├── ModelRenderer.js
│   │   ├── TerrainRenderer.js
│   │   └── CameraController.js
│   ├── entities/
│   │   ├── Player.js
│   │   ├── NPC.js
│   │   └── GameObject.js
│   ├── systems/
│   │   ├── AnimationController.js
│   │   ├── LODManager.js
│   │   └── ChunkManager.js
│   └── ui/
│       ├── Minimap.js
│       ├── TabInterface.js
│       └── ChatBox.js
└── assets/
    └── data_caches/560/      # Your cache files
```

---

## Conclusion

This 3D WebGL client will provide an authentic RuneScape experience in the browser, matching the visual quality shown in your reference screenshots. By leveraging your Build 560 cache data, we can render actual RuneScape models and textures from 2009.

**Key Advantages:**
- Authentic RS look (matches screenshots)
- Uses real game assets (cache data)
- Performant (60 FPS target)
- Backwards compatible (2D fallback)
- Maintainable architecture

**Next Steps:**
1. Review this specification
2. Decide: 3D path or stick with 2D?
3. If 3D: Start Phase 1 foundation
4. Commission/create missing assets if needed

This is a significant undertaking but will result in a professional-quality OSRS-style client that rivals RuneLite and commercial clients.