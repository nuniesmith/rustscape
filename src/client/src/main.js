/**
 * Rustscape 3D Client - Main Entry Point
 * Initializes Three.js, loads cache data, and starts the game loop
 */

import * as THREE from "three";
import {
    CacheReader,
    ArchiveType,
    getArchiveName,
} from "@cache/CacheReader.js";
import {
    ModelParser,
    PlayerModelIds,
    NpcModelIds,
} from "@cache/ModelParser.js";
import { PlayerModel } from "@entities/PlayerModel.js";
import { NPC } from "@entities/NPC.js";

// Game state
const gameState = {
    loaded: false,
    scene: null,
    camera: null,
    renderer: null,
    clock: new THREE.Clock(),

    // Cache system
    cacheReader: null,
    modelParser: null,

    // Player
    playerId: null,
    playerUsername: null,
    playerPosition: { x: 3222, y: 3218, z: 0 },

    // Entities
    players: new Map(),
    npcs: new Map(),
    objects: new Map(),

    // Network
    ws: null,
    connected: false,

    // Camera control
    cameraRotation: 0, // 0, 90, 180, 270 degrees
    cameraZoom: 1.0,

    // Performance
    fps: 0,
    frameCount: 0,
    lastFpsUpdate: 0,
};

// Initialize the game
async function init() {
    console.log("🎮 Rustscape 3D Client Starting...");

    // Initialize cache system
    gameState.cacheReader = new CacheReader("/assets/data_caches/560");
    gameState.modelParser = new ModelParser();

    updateLoadingProgress(10, "Initializing renderer...");

    // Set up Three.js renderer
    const canvas = document.getElementById("game-canvas");
    const renderer = new THREE.WebGLRenderer({
        canvas: canvas,
        antialias: true,
        alpha: false,
        powerPreference: "high-performance",
    });

    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.setSize(canvas.clientWidth, canvas.clientHeight);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    renderer.setClearColor(0x9eb4b8); // Sky blue

    gameState.renderer = renderer;

    updateLoadingProgress(20, "Creating scene...");

    // Create scene
    const scene = new THREE.Scene();
    scene.fog = new THREE.Fog(0x9eb4b8, 50, 100);
    gameState.scene = scene;

    updateLoadingProgress(30, "Setting up camera...");

    // Create isometric camera
    const camera = createIsometricCamera();
    gameState.camera = camera;

    updateLoadingProgress(40, "Adding lighting...");

    // Set up lighting
    setupLighting(scene);

    updateLoadingProgress(50, "Loading cache data...");

    // Load cache archives
    try {
        await loadCacheData();
        updateLoadingProgress(60, "Loading player model...");
    } catch (error) {
        console.error("❌ Failed to load cache:", error);
        addChatMessage("Cache loading failed, using test models", "server");
        updateLoadingProgress(60, "Using fallback models...");
    }

    // Create terrain
    await createTerrain(scene);

    updateLoadingProgress(70, "Creating player...");

    // Create player
    await createPlayer(scene);

    updateLoadingProgress(75, "Spawning NPCs...");

    // Spawn test NPCs
    spawnTestNPCs(scene);

    updateLoadingProgress(80, "Connecting to server...");

    // Set up WebSocket connection
    setupNetworking();

    updateLoadingProgress(90, "Initializing UI...");

    // Set up UI
    setupUI();

    updateLoadingProgress(95, "Setting up controls...");

    // Set up controls
    setupControls(canvas);

    updateLoadingProgress(100, "Ready!");

    // Hide loading screen
    setTimeout(() => {
        document.getElementById("loading-screen").classList.add("hidden");
        gameState.loaded = true;
        addChatMessage("Welcome to Rustscape!", "server");
        addChatMessage("Click on the ground to move around.", "game");
    }, 500);

    // Start game loop
    animate();

    console.log("✅ Rustscape 3D Client Ready");
}

// Create isometric camera
function createIsometricCamera() {
    const aspect = (window.innerWidth * 0.8) / window.innerHeight;
    const frustumSize = 20;

    const camera = new THREE.OrthographicCamera(
        (frustumSize * aspect) / -2, // left
        (frustumSize * aspect) / 2, // right
        frustumSize / 2, // top
        frustumSize / -2, // bottom
        0.1, // near
        1000, // far
    );

    // Position for isometric view (like RuneScape)
    camera.position.set(15, 15, 15);
    camera.lookAt(0, 0, 0);

    return camera;
}

// Set up lighting
function setupLighting(scene) {
    // Ambient light (fills shadows)
    const ambient = new THREE.AmbientLight(0x666666);
    scene.add(ambient);

    // Directional light (sun) - classic RS lighting angle
    const sun = new THREE.DirectionalLight(0xffffee, 0.8);
    sun.position.set(50, 100, 50);
    sun.castShadow = true;
    sun.shadow.mapSize.width = 2048;
    sun.shadow.mapSize.height = 2048;
    sun.shadow.camera.near = 0.5;
    sun.shadow.camera.far = 500;
    scene.add(sun);
}

/**
 * Load cache data
 */
async function loadCacheData() {
    try {
        console.log("📦 Loading cache archives...");

        // Pre-load essential archives
        await gameState.cacheReader.loadDataFile();

        addChatMessage("Cache loaded successfully!", "server");
    } catch (error) {
        console.error("Failed to load cache:", error);
        throw error;
    }
}

/**
 * Create terrain (using cache data or fallback)
 */
async function createTerrain(scene) {
    try {
        // Try to load terrain from cache
        console.log("🗺️ Loading terrain from cache...");

        // For now, use test terrain
        // TODO: Implement actual cache terrain loading
        createTestTerrain(scene);
    } catch (error) {
        console.warn(
            "⚠️ Failed to load terrain from cache, using test terrain",
        );
        createTestTerrain(scene);
    }
}

/**
 * Create test terrain (fallback)
 */
function createTestTerrain(scene) {
    const geometry = new THREE.PlaneGeometry(100, 100, 50, 50);

    // Add some height variation
    const positions = geometry.attributes.position.array;
    for (let i = 0; i < positions.length; i += 3) {
        const x = positions[i];
        const y = positions[i + 1];
        positions[i + 2] = Math.sin(x * 0.1) * Math.cos(y * 0.1) * 2; // Z is height
    }
    geometry.computeVertexNormals();

    const material = new THREE.MeshLambertMaterial({
        color: 0x3a6b1f, // Grass green
        flatShading: true, // Low-poly RS look
    });

    const terrain = new THREE.Mesh(geometry, material);
    terrain.rotation.x = -Math.PI / 2; // Lay flat
    terrain.receiveShadow = true;
    terrain.position.y = -0.5;

    scene.add(terrain);

    // Add grid helper for development
    const gridHelper = new THREE.GridHelper(100, 100, 0x444444, 0x222222);
    gridHelper.position.y = 0;
    scene.add(gridHelper);
}

/**
 * Create player using PlayerModel class
 */
async function createPlayer(scene) {
    try {
        console.log("👤 Creating player model...");

        // Create player model instance
        const playerModel = new PlayerModel(
            gameState.playerUsername || "Player1",
            {
                x: gameState.playerPosition.x / 10,
                y: 0,
                z: gameState.playerPosition.y / 10,
            },
        );

        // Set cache readers
        playerModel.setCache(gameState.cacheReader, gameState.modelParser);

        // Load the model
        const playerGroup = await playerModel.load();

        // Add to scene
        scene.add(playerGroup);
        gameState.player = playerModel;

        addChatMessage("✅ Player model loaded!", "server");

        // Test equipment (optional)
        testEquipment(playerModel);

        return playerModel;
    } catch (error) {
        console.error("⚠️ Failed to create player:", error);
        addChatMessage("Failed to create player model", "server");
        return null;
    }
}

/**
 * Test equipment system (demo)
 */
function testEquipment(playerModel) {
    // Equip test items after a delay to show the system working
    setTimeout(() => {
        console.log("🗡️ Equipping test items...");
        playerModel.setEquipment("helmet", 1);
        playerModel.setEquipment("weapon", 2);
        playerModel.setEquipment("shield", 3);
        addChatMessage("Equipped test gear!", "system");
    }, 2000);
}

/**
 * Spawn test NPCs around the player
 */
function spawnTestNPCs(scene) {
    console.log("🧙 Spawning test NPCs...");

    const npcData = [
        { id: 1, name: "Shopkeeper", x: 5, z: 5, behavior: "idle" },
        { id: 2, name: "Guard", x: -5, z: 5, behavior: "wander" },
        { id: 3, name: "Goblin", x: 0, z: 10, behavior: "wander" },
        { id: 4, name: "Chicken", x: 8, z: -3, behavior: "wander" },
    ];

    npcData.forEach(async (data) => {
        const npc = new NPC(data.id, data.id, data.name, {
            x: data.x,
            y: 0,
            z: data.z,
        });

        npc.setCache(gameState.cacheReader, gameState.modelParser);
        npc.setBehavior(data.behavior);

        const npcGroup = await npc.load();
        scene.add(npcGroup);
        gameState.npcs.set(data.id, npc);
    });

    addChatMessage(`Spawned ${npcData.length} NPCs`, "system");
}

/**
 * Add name label to a group
 */
function addNameLabel(group, name) {
    const canvas = document.createElement("canvas");
    canvas.width = 256;
    canvas.height = 64;
    const ctx = canvas.getContext("2d");
    ctx.font = "bold 24px Courier New";
    ctx.fillStyle = "#ffffff";
    ctx.strokeStyle = "#000000";
    ctx.lineWidth = 3;
    ctx.textAlign = "center";
    ctx.strokeText(name, 128, 40);
    ctx.fillText(name, 128, 40);

    const texture = new THREE.CanvasTexture(canvas);
    const spriteMaterial = new THREE.SpriteMaterial({ map: texture });
    const sprite = new THREE.Sprite(spriteMaterial);
    sprite.scale.set(2, 0.5, 1);
    sprite.position.y = 3.5;
    group.add(sprite);
}

// Update camera to follow player
function updateCamera() {
    if (!gameState.player || !gameState.camera) return;

    const distance = 20 / gameState.cameraZoom;
    const angleRad = (gameState.cameraRotation * Math.PI) / 180;
    const pitchRad = (26.565 * Math.PI) / 180; // Isometric angle

    // Get player position (works with both PlayerModel and old player objects)
    const targetPos =
        gameState.player.position || gameState.player.getGroup().position;

    gameState.camera.position.x =
        targetPos.x + distance * Math.cos(angleRad) * Math.cos(pitchRad);
    gameState.camera.position.y = targetPos.y + distance * Math.sin(pitchRad);
    gameState.camera.position.z =
        targetPos.z + distance * Math.sin(angleRad) * Math.cos(pitchRad);

    gameState.camera.lookAt(targetPos.x, targetPos.y, targetPos.z);
}

// Set up networking
function setupNetworking() {
    // This will connect to the existing Rustscape server
    // For now, just log that we're ready
    console.log("🌐 Ready to connect to server");
    addChatMessage("Ready to connect. Type /connect to join server.", "game");
}

// Set up UI
function setupUI() {
    // Tab switching
    const tabButtons = document.querySelectorAll(".tab-btn");
    const tabPanels = document.querySelectorAll(".tab-panel");

    tabButtons.forEach((button) => {
        button.addEventListener("click", () => {
            const tabName = button.dataset.tab;

            // Update buttons
            tabButtons.forEach((btn) => btn.classList.remove("active"));
            button.classList.add("active");

            // Update panels
            tabPanels.forEach((panel) => panel.classList.remove("active"));
            document.getElementById(`${tabName}-panel`).classList.add("active");
        });
    });

    // Chat tabs
    const chatTabs = document.querySelectorAll(".chat-tab");
    chatTabs.forEach((tab) => {
        tab.addEventListener("click", () => {
            chatTabs.forEach((t) => t.classList.remove("active"));
            tab.classList.add("active");
        });
    });

    // Chat input
    const chatInput = document.getElementById("chat-input");
    chatInput.addEventListener("keypress", (e) => {
        if (e.key === "Enter" && chatInput.value.trim()) {
            handleChatMessage(chatInput.value.trim());
            chatInput.value = "";
        }
    });

    // Create inventory slots
    const inventoryGrid = document.getElementById("inventory-grid");
    for (let i = 0; i < 28; i++) {
        const slot = document.createElement("div");
        slot.className = "inventory-slot";
        slot.dataset.slot = i;
        inventoryGrid.appendChild(slot);
    }

    // Compass rotation
    const compass = document.getElementById("compass");
    compass.addEventListener("click", () => {
        gameState.cameraRotation = (gameState.cameraRotation + 90) % 360;
        updateCompass();
    });
}

// Set up controls
function setupControls(canvas) {
    // Mouse wheel for zoom
    canvas.addEventListener("wheel", (e) => {
        e.preventDefault();
        gameState.cameraZoom += e.deltaY * -0.001;
        gameState.cameraZoom = Math.max(
            0.5,
            Math.min(2.0, gameState.cameraZoom),
        );
    });

    // Click to move (will be implemented with raycasting)
    canvas.addEventListener("click", (e) => {
        const rect = canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        // For now, just log
        console.log(`Clicked at (${x}, ${y})`);
        addChatMessage("Click-to-move will be implemented soon!", "game");
    });

    // Keyboard controls
    window.addEventListener("keydown", (e) => {
        switch (e.key) {
            case "ArrowLeft":
                gameState.cameraRotation =
                    (gameState.cameraRotation + 90) % 360;
                updateCompass();
                break;
            case "ArrowRight":
                gameState.cameraRotation =
                    (gameState.cameraRotation - 90 + 360) % 360;
                updateCompass();
                break;
        }
    });

    // Window resize
    window.addEventListener("resize", onWindowResize);
}

// Handle window resize
function onWindowResize() {
    const canvas = document.getElementById("game-canvas");
    const aspect = canvas.clientWidth / canvas.clientHeight;
    const frustumSize = 20;

    gameState.camera.left = (frustumSize * aspect) / -2;
    gameState.camera.right = (frustumSize * aspect) / 2;
    gameState.camera.top = frustumSize / 2;
    gameState.camera.bottom = frustumSize / -2;
    gameState.camera.updateProjectionMatrix();

    gameState.renderer.setSize(canvas.clientWidth, canvas.clientHeight);
}

// Update compass display
function updateCompass() {
    const compass = document.getElementById("compass");
    const directions = ["N", "E", "S", "W"];
    const index = (gameState.cameraRotation / 90) % 4;
    compass.textContent = directions[index];
}

// Add chat message
function addChatMessage(text, type = "game") {
    const messagesDiv = document.getElementById("chat-messages");
    const messageEl = document.createElement("div");
    messageEl.className = `chat-message ${type}`;
    messageEl.textContent = text;
    messagesDiv.appendChild(messageEl);
    messagesDiv.scrollTop = messagesDiv.scrollHeight;
}

// Handle chat message
function handleChatMessage(message) {
    if (message.startsWith("/")) {
        // Command
        handleCommand(message);
    } else {
        // Regular chat
        addChatMessage(
            `${gameState.playerUsername || "Player1"}: ${message}`,
            "player",
        );
    }
}

// Handle commands
function handleCommand(command) {
    const parts = command.slice(1).toLowerCase().split(" ");
    const cmd = parts[0];

    switch (cmd) {
        case "connect":
            addChatMessage("Connecting to server...", "game");
            // TODO: Implement actual connection
            break;
        case "help":
            addChatMessage(
                "Commands: /connect, /help, /fps, /cache, /reload",
                "game",
            );
            break;
        case "fps":
            const fpsCounter = document.getElementById("fps-counter");
            fpsCounter.style.display =
                fpsCounter.style.display === "none" ? "block" : "none";
            break;
        case "cache":
            const cacheStats = gameState.cacheReader.getStats();
            const modelStats = gameState.modelParser.getStats();
            addChatMessage(
                `Cache: ${cacheStats.archivesLoaded} archives, ${cacheStats.memoryUsage}`,
                "game",
            );
            addChatMessage(
                `Models: ${modelStats.cachedModels} cached, ${modelStats.memoryEstimate}`,
                "game",
            );
            break;
        case "reload":
            addChatMessage("Reloading player model...", "game");
            reloadPlayer();
            break;
        default:
            addChatMessage(`Unknown command: ${command}`, "game");
    }
}

// Reload player model
async function reloadPlayer() {
    if (gameState.player) {
        gameState.scene.remove(gameState.player);
    }
    await createPlayer(gameState.scene);
    addChatMessage("Player model reloaded!", "game");
}

// Update loading progress
function updateLoadingProgress(percent, text) {
    const progressBar = document.getElementById("loading-progress");
    const loadingText = document.getElementById("loading-text");

    progressBar.style.width = `${percent}%`;
    loadingText.textContent = text;
}

// Update FPS counter
function updateFPS() {
    gameState.frameCount++;
    const now = performance.now();

    if (now >= gameState.lastFpsUpdate + 1000) {
        gameState.fps = Math.round(
            (gameState.frameCount * 1000) / (now - gameState.lastFpsUpdate),
        );
        gameState.frameCount = 0;
        gameState.lastFpsUpdate = now;

        const fpsCounter = document.getElementById("fps-counter");
        fpsCounter.textContent = `FPS: ${gameState.fps}`;

        // Color code FPS
        if (gameState.fps >= 50) {
            fpsCounter.style.color = "#0f0";
        } else if (gameState.fps >= 30) {
            fpsCounter.style.color = "#ff0";
        } else {
            fpsCounter.style.color = "#f00";
        }
    }
}

// Main animation loop
function animate() {
    requestAnimationFrame(animate);

    if (!gameState.loaded) return;

    const delta = gameState.clock.getDelta();

    // Update player
    if (gameState.player && gameState.player.update) {
        gameState.player.update(delta);
    }

    // Update all NPCs
    gameState.npcs.forEach((npc) => {
        npc.update(delta);
    });

    // Update camera position
    updateCamera();

    // Update FPS
    updateFPS();

    // Render scene
    gameState.renderer.render(gameState.scene, gameState.camera);
}

// Start the game when page loads
window.addEventListener("DOMContentLoaded", init);

// Export for debugging
window.gameState = gameState;
