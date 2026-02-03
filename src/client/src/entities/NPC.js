/**
 * NPC.js
 *
 * Represents a Non-Player Character in the game world
 * - Loads NPC models from cache
 * - Manages NPC appearance and equipment
 * - Handles AI behavior and movement
 * - Name labels and interactions
 *
 * Phase 3: Enhanced Rendering
 */

import * as THREE from 'three';

export class NPC {
    constructor(id, npcId, name, position = { x: 0, y: 0, z: 0 }) {
        this.id = id;              // Unique instance ID
        this.npcId = npcId;        // NPC definition ID (from cache)
        this.name = name;
        this.position = position;

        // Main container
        this.group = new THREE.Group();
        this.group.position.set(position.x, position.y, position.z);
        this.group.userData = { type: 'npc', id: this.id };

        // Model data
        this.mesh = null;
        this.nameLabel = null;

        // Movement and AI
        this.velocity = { x: 0, y: 0, z: 0 };
        this.targetPosition = null;
        this.moveSpeed = 1.5;
        this.rotationSpeed = 5.0;

        // Animation
        this.animationState = 'idle';
        this.animationTime = 0;

        // Combat
        this.health = 100;
        this.maxHealth = 100;
        this.combatLevel = 1;

        // Interaction
        this.isInteractable = true;
        this.actions = ['Talk-to', 'Attack'];

        // Cache references
        this.cacheReader = null;
        this.modelParser = null;

        // AI behavior
        this.behavior = 'idle';  // idle, wander, follow, hostile
        this.wanderRadius = 5;
        this.wanderTimer = 0;
        this.wanderInterval = 5; // Wander every 5 seconds
    }

    /**
     * Initialize with cache readers
     */
    setCache(cacheReader, modelParser) {
        this.cacheReader = cacheReader;
        this.modelParser = modelParser;
    }

    /**
     * Load NPC model from cache
     */
    async load() {
        console.log(`[NPC] Loading NPC ${this.name} (ID: ${this.npcId})`);

        try {
            // Try to load actual model from cache
            if (this.cacheReader && this.modelParser) {
                await this.loadFromCache();
            } else {
                // Fallback to test model
                this.createTestModel();
            }

            // Create name label
            this.createNameLabel();

            console.log(`[NPC] Loaded ${this.name}`);
            return this.group;

        } catch (err) {
            console.warn(`[NPC] Failed to load from cache, using test model:`, err);
            this.createTestModel();
            this.createNameLabel();
            return this.group;
        }
    }

    /**
     * Load NPC model from cache
     */
    async loadFromCache() {
        // TODO: Implement actual NPC loading from cache
        // NPCs are stored in Archive 1 (NPC definitions)
        // Model IDs are referenced in the NPC definition

        // For now, create a test model
        // In production, this would:
        // 1. Load NPC definition from cache
        // 2. Get model IDs from definition
        // 3. Load and combine model parts
        // 4. Apply colors/textures

        this.createTestModel();
    }

    /**
     * Create a simple test model
     */
    createTestModel() {
        // Create a simple colored body to distinguish NPCs
        const bodyGeometry = new THREE.BoxGeometry(0.6, 1.2, 0.4);

        // Color based on NPC ID for variety
        const hue = (this.npcId * 137.5) % 360;
        const color = new THREE.Color().setHSL(hue / 360, 0.7, 0.5);

        const bodyMaterial = new THREE.MeshLambertMaterial({
            color: color,
            flatShading: true
        });

        const body = new THREE.Mesh(bodyGeometry, bodyMaterial);
        body.position.y = 0.6;
        body.castShadow = true;
        body.receiveShadow = true;
        this.group.add(body);

        // Head
        const headGeometry = new THREE.BoxGeometry(0.5, 0.5, 0.5);
        const headMaterial = new THREE.MeshLambertMaterial({
            color: 0xFFDBAC,  // Skin tone
            flatShading: true
        });
        const head = new THREE.Mesh(headGeometry, headMaterial);
        head.position.y = 1.45;
        head.castShadow = true;
        this.group.add(head);

        // Arms (simplified)
        const armGeometry = new THREE.BoxGeometry(0.2, 0.8, 0.2);
        const armMaterial = new THREE.MeshLambertMaterial({
            color: color,
            flatShading: true
        });

        const leftArm = new THREE.Mesh(armGeometry, armMaterial);
        leftArm.position.set(-0.45, 0.6, 0);
        leftArm.castShadow = true;
        this.group.add(leftArm);

        const rightArm = new THREE.Mesh(armGeometry, armMaterial.clone());
        rightArm.position.set(0.45, 0.6, 0);
        rightArm.castShadow = true;
        this.group.add(rightArm);

        // Legs
        const legGeometry = new THREE.BoxGeometry(0.25, 0.8, 0.25);
        const legMaterial = new THREE.MeshLambertMaterial({
            color: 0x4169E1,
            flatShading: true
        });

        const leftLeg = new THREE.Mesh(legGeometry, legMaterial);
        leftLeg.position.set(-0.15, -0.2, 0);
        leftLeg.castShadow = true;
        this.group.add(leftLeg);

        const rightLeg = new THREE.Mesh(legGeometry, legMaterial.clone());
        rightLeg.position.set(0.15, -0.2, 0);
        rightLeg.castShadow = true;
        this.group.add(rightLeg);

        this.mesh = body; // Reference to main body
    }

    /**
     * Create name label with combat level
     */
    createNameLabel() {
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        canvas.width = 256;
        canvas.height = 64;

        // Background (optional, for readability)
        context.fillStyle = 'rgba(0, 0, 0, 0.3)';
        context.fillRect(0, 10, 256, 44);

        // NPC name (yellow for NPCs)
        context.fillStyle = '#FFFF00';
        context.font = 'Bold 28px Arial';
        context.textAlign = 'center';
        context.fillText(this.name, 128, 32);

        // Combat level (smaller, below name)
        context.fillStyle = '#00FF00';
        context.font = '20px Arial';
        context.fillText(`(level-${this.combatLevel})`, 128, 50);

        // Create sprite
        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({
            map: texture,
            transparent: true
        });
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.scale.set(2.5, 0.6, 1);
        sprite.position.y = 2.3;

        this.nameLabel = sprite;
        this.group.add(sprite);
    }

    /**
     * Update NPC AI and animation
     */
    update(delta) {
        this.animationTime += delta;

        // Update AI behavior
        this.updateBehavior(delta);

        // Update movement
        this.updateMovement(delta);

        // Update animation
        this.updateAnimation(delta);
    }

    /**
     * Update AI behavior
     */
    updateBehavior(delta) {
        switch (this.behavior) {
            case 'idle':
                // Do nothing, just stand still
                break;

            case 'wander':
                this.wanderTimer += delta;
                if (this.wanderTimer >= this.wanderInterval) {
                    this.wanderTimer = 0;
                    this.chooseWanderTarget();
                }
                break;

            case 'follow':
                // TODO: Follow player
                break;

            case 'hostile':
                // TODO: Attack nearby players
                break;
        }
    }

    /**
     * Choose a random wander target within radius
     */
    chooseWanderTarget() {
        const angle = Math.random() * Math.PI * 2;
        const distance = Math.random() * this.wanderRadius;

        this.targetPosition = {
            x: this.position.x + Math.cos(angle) * distance,
            y: this.position.y,
            z: this.position.z + Math.sin(angle) * distance
        };

        this.animationState = 'walk';
    }

    /**
     * Update movement towards target
     */
    updateMovement(delta) {
        if (!this.targetPosition) return;

        const dx = this.targetPosition.x - this.group.position.x;
        const dz = this.targetPosition.z - this.group.position.z;
        const distance = Math.sqrt(dx * dx + dz * dz);

        // Reached target
        if (distance < 0.1) {
            this.targetPosition = null;
            this.animationState = 'idle';
            return;
        }

        // Move towards target
        const moveAmount = Math.min(this.moveSpeed * delta, distance);
        const dirX = dx / distance;
        const dirZ = dz / distance;

        this.group.position.x += dirX * moveAmount;
        this.group.position.z += dirZ * moveAmount;

        // Update position
        this.position.x = this.group.position.x;
        this.position.z = this.group.position.z;

        // Rotate to face movement direction
        const targetRotation = Math.atan2(dx, dz);
        this.group.rotation.y = this.lerpAngle(
            this.group.rotation.y,
            targetRotation,
            this.rotationSpeed * delta
        );
    }

    /**
     * Lerp between angles (handles wrapping)
     */
    lerpAngle(from, to, t) {
        let diff = to - from;
        while (diff > Math.PI) diff -= Math.PI * 2;
        while (diff < -Math.PI) diff += Math.PI * 2;
        return from + diff * t;
    }

    /**
     * Update animation state
     */
    updateAnimation(delta) {
        if (this.animationState === 'idle') {
            // Simple breathing/idle animation
            const breathe = Math.sin(this.animationTime * 2) * 0.02;
            if (this.mesh) {
                this.mesh.scale.y = 1 + breathe;
            }
        }
        else if (this.animationState === 'walk') {
            // Simple bobbing while walking
            const bob = Math.sin(this.animationTime * 8) * 0.05;
            this.group.position.y = this.position.y + bob;
        }
    }

    /**
     * Set NPC behavior
     */
    setBehavior(behavior) {
        this.behavior = behavior;
        console.log(`[NPC] ${this.name} behavior: ${behavior}`);
    }

    /**
     * Move to specific position
     */
    moveTo(x, y, z) {
        this.targetPosition = { x, y, z };
        this.animationState = 'walk';
    }

    /**
     * Take damage
     */
    takeDamage(amount) {
        this.health = Math.max(0, this.health - amount);
        console.log(`[NPC] ${this.name} took ${amount} damage (${this.health}/${this.maxHealth})`);

        if (this.health <= 0) {
            this.die();
        }
    }

    /**
     * Handle NPC death
     */
    die() {
        console.log(`[NPC] ${this.name} died`);
        this.animationState = 'death';
        // TODO: Death animation, loot drop, respawn timer
    }

    /**
     * Get distance to point
     */
    distanceTo(x, y, z) {
        const dx = this.position.x - x;
        const dy = this.position.y - y;
        const dz = this.position.z - z;
        return Math.sqrt(dx * dx + dy * dy + dz * dz);
    }

    /**
     * Check if point is within interaction range
     */
    isInRange(x, y, z, range = 2) {
        return this.distanceTo(x, y, z) <= range;
    }

    /**
     * Handle interaction
     */
    interact(action) {
        console.log(`[NPC] Player ${action} ${this.name}`);

        switch (action) {
            case 'Talk-to':
                return {
                    type: 'dialogue',
                    npc: this.name,
                    message: `Hello, I am ${this.name}!`
                };

            case 'Attack':
                return {
                    type: 'combat',
                    npc: this.name,
                    target: this.id
                };

            default:
                return null;
        }
    }

    /**
     * Get available actions for right-click menu
     */
    getActions() {
        return this.actions.map(action => ({
            text: action,
            color: action === 'Attack' ? '#FF0000' : '#FFFFFF'
        }));
    }

    /**
     * Remove from scene
     */
    destroy() {
        // Clean up meshes
        this.group.traverse((child) => {
            if (child.geometry) child.geometry.dispose();
            if (child.material) child.material.dispose();
        });

        // Remove from parent
        if (this.group.parent) {
            this.group.parent.remove(this.group);
        }
    }

    /**
     * Get Three.js group
     */
    getGroup() {
        return this.group;
    }
}
