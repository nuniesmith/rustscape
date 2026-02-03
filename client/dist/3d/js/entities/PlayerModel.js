/**
 * PlayerModel.js
 *
 * Manages complete player rendering including:
 * - Multi-part body (head, torso, arms, legs)
 * - Equipment overlay (helmet, chest, legs, weapon, shield)
 * - Animations (idle, walk, run)
 * - Name labels
 *
 * Phase 3: Enhanced Rendering
 */

import * as THREE from 'three';

export class PlayerModel {
    constructor(username, position = { x: 0, y: 0, z: 0 }) {
        this.username = username;
        this.position = position;

        // Main container for entire player
        this.group = new THREE.Group();
        this.group.position.set(position.x, position.y, position.z);

        // Body parts (loaded from cache)
        this.bodyParts = {
            head: null,
            torso: null,
            arms: null,
            legs: null,
            feet: null,
            hands: null
        };

        // Equipment (layered on top of body)
        this.equipment = {
            helmet: null,      // Slot 0
            cape: null,        // Slot 1
            amulet: null,      // Slot 2
            weapon: null,      // Slot 3
            chest: null,       // Slot 4
            shield: null,      // Slot 5
            legs: null,        // Slot 7
            gloves: null,      // Slot 9
            boots: null,       // Slot 10
            ring: null,        // Slot 12
            ammo: null         // Slot 13
        };

        // Animation state
        this.animationState = 'idle';
        this.animationTime = 0;

        // Name label
        this.nameLabel = null;

        // Cache references (injected)
        this.cacheReader = null;
        this.modelParser = null;

        // Gender (0 = male, 1 = female)
        this.gender = 0;

        // Appearance (colors for body parts)
        this.appearance = {
            hairColor: 0,
            torsoColor: 0,
            legColor: 0,
            feetColor: 0,
            skinColor: 0
        };
    }

    /**
     * Initialize with cache readers
     */
    setCache(cacheReader, modelParser) {
        this.cacheReader = cacheReader;
        this.modelParser = modelParser;
    }

    /**
     * Load complete player model
     * This loads the base body and any equipped items
     */
    async load() {
        console.log(`[PlayerModel] Loading model for ${this.username}`);

        // Load body parts first
        await this.loadBodyParts();

        // Load any equipped items
        await this.loadEquipment();

        // Add name label
        this.createNameLabel();

        console.log(`[PlayerModel] Loaded ${this.username}`);
        return this.group;
    }

    /**
     * Load base body parts (no equipment)
     */
    async loadBodyParts() {
        // OSRS character model IDs (Build 560)
        // These are the default male body part models
        const bodyPartIds = {
            head: 0,      // Model 0 = male head
            torso: 18,    // Model 18 = male torso
            arms: 26,     // Model 26 = male arms
            legs: 36,     // Model 36 = male legs
            feet: 42,     // Model 42 = male feet
            hands: 33     // Model 33 = male hands
        };

        // Female variants (add 65 to each ID)
        if (this.gender === 1) {
            bodyPartIds.head = 45;
            bodyPartIds.torso = 56;
            bodyPartIds.arms = 61;
            bodyPartIds.legs = 64;
            bodyPartIds.feet = 79;
            bodyPartIds.hands = 65;
        }

        // For now, create a simple placeholder body
        // In production, we'd load actual models from cache
        await this.createTestBody();

        // TODO: Load actual models when cache reader supports it
        // for (const [partName, modelId] of Object.entries(bodyPartIds)) {
        //     try {
        //         const modelData = await this.cacheReader.getModel(modelId);
        //         const mesh = this.modelParser.parse(modelData);
        //         this.bodyParts[partName] = mesh;
        //         this.group.add(mesh);
        //     } catch (err) {
        //         console.warn(`Failed to load ${partName}:`, err);
        //     }
        // }
    }

    /**
     * Create a simple test body (placeholder until cache loading works)
     */
    async createTestBody() {
        const group = new THREE.Group();

        // Body (torso)
        const bodyGeometry = new THREE.BoxGeometry(0.6, 1.2, 0.4);
        const bodyMaterial = new THREE.MeshLambertMaterial({
            color: 0x8B4513,  // Brown
            flatShading: true
        });
        const body = new THREE.Mesh(bodyGeometry, bodyMaterial);
        body.position.y = 0.6;
        body.castShadow = true;
        body.receiveShadow = true;
        group.add(body);
        this.bodyParts.torso = body;

        // Head
        const headGeometry = new THREE.BoxGeometry(0.5, 0.5, 0.5);
        const headMaterial = new THREE.MeshLambertMaterial({
            color: 0xFFDBAC,  // Skin tone
            flatShading: true
        });
        const head = new THREE.Mesh(headGeometry, headMaterial);
        head.position.y = 1.45;
        head.castShadow = true;
        head.receiveShadow = true;
        group.add(head);
        this.bodyParts.head = head;

        // Arms (left)
        const armGeometry = new THREE.BoxGeometry(0.25, 0.8, 0.25);
        const armMaterial = new THREE.MeshLambertMaterial({
            color: 0x8B4513,  // Match body
            flatShading: true
        });
        const leftArm = new THREE.Mesh(armGeometry, armMaterial);
        leftArm.position.set(-0.5, 0.6, 0);
        leftArm.castShadow = true;
        group.add(leftArm);

        // Arms (right)
        const rightArm = new THREE.Mesh(armGeometry, armMaterial.clone());
        rightArm.position.set(0.5, 0.6, 0);
        rightArm.castShadow = true;
        group.add(rightArm);
        this.bodyParts.arms = { left: leftArm, right: rightArm };

        // Legs (left)
        const legGeometry = new THREE.BoxGeometry(0.28, 0.8, 0.28);
        const legMaterial = new THREE.MeshLambertMaterial({
            color: 0x4169E1,  // Blue pants
            flatShading: true
        });
        const leftLeg = new THREE.Mesh(legGeometry, legMaterial);
        leftLeg.position.set(-0.16, -0.2, 0);
        leftLeg.castShadow = true;
        group.add(leftLeg);

        // Legs (right)
        const rightLeg = new THREE.Mesh(legGeometry, legMaterial.clone());
        rightLeg.position.set(0.16, -0.2, 0);
        rightLeg.castShadow = true;
        group.add(rightLeg);
        this.bodyParts.legs = { left: leftLeg, right: rightLeg };

        this.group.add(group);
    }

    /**
     * Load equipment models
     */
    async loadEquipment() {
        // Equipment will be loaded on-demand when setEquipment() is called
        // This is just a placeholder for future expansion
        console.log(`[PlayerModel] Equipment slots ready`);
    }

    /**
     * Equip an item in a specific slot
     * @param {string} slot - Equipment slot name (helmet, weapon, etc.)
     * @param {number} itemId - Item ID from cache
     */
    async setEquipment(slot, itemId) {
        if (!this.equipment.hasOwnProperty(slot)) {
            console.warn(`[PlayerModel] Invalid equipment slot: ${slot}`);
            return;
        }

        // Remove existing equipment in this slot
        if (this.equipment[slot]) {
            this.group.remove(this.equipment[slot]);
            this.equipment[slot] = null;
        }

        if (itemId === null || itemId === -1) {
            // Unequip
            return;
        }

        try {
            // Load item model from cache
            console.log(`[PlayerModel] Loading equipment: ${slot} (ID: ${itemId})`);

            // TODO: Load actual equipment model from cache
            // For now, create a placeholder
            const equipMesh = this.createTestEquipment(slot, itemId);

            if (equipMesh) {
                this.equipment[slot] = equipMesh;
                this.group.add(equipMesh);
                console.log(`[PlayerModel] Equipped ${slot}`);
            }

        } catch (err) {
            console.error(`[PlayerModel] Failed to load equipment ${slot}:`, err);
        }
    }

    /**
     * Create test equipment (placeholder)
     */
    createTestEquipment(slot, itemId) {
        let mesh = null;

        switch (slot) {
            case 'helmet':
                // Simple helmet placeholder
                const helmetGeometry = new THREE.BoxGeometry(0.55, 0.4, 0.55);
                const helmetMaterial = new THREE.MeshLambertMaterial({
                    color: 0x808080,  // Gray
                    flatShading: true
                });
                mesh = new THREE.Mesh(helmetGeometry, helmetMaterial);
                mesh.position.y = 1.6;
                mesh.castShadow = true;
                break;

            case 'weapon':
                // Simple sword placeholder
                const swordGeometry = new THREE.BoxGeometry(0.1, 1.0, 0.1);
                const swordMaterial = new THREE.MeshLambertMaterial({
                    color: 0xC0C0C0,  // Silver
                    flatShading: true
                });
                mesh = new THREE.Mesh(swordGeometry, swordMaterial);
                mesh.position.set(0.5, 0.4, 0);
                mesh.rotation.z = Math.PI / 4;
                mesh.castShadow = true;
                break;

            case 'shield':
                // Simple shield placeholder
                const shieldGeometry = new THREE.BoxGeometry(0.4, 0.6, 0.1);
                const shieldMaterial = new THREE.MeshLambertMaterial({
                    color: 0x8B0000,  // Dark red
                    flatShading: true
                });
                mesh = new THREE.Mesh(shieldGeometry, shieldMaterial);
                mesh.position.set(-0.5, 0.6, 0);
                mesh.castShadow = true;
                break;

            case 'chest':
                // Armor overlay on torso
                const chestGeometry = new THREE.BoxGeometry(0.65, 1.25, 0.45);
                const chestMaterial = new THREE.MeshLambertMaterial({
                    color: 0x4A4A4A,  // Dark gray armor
                    flatShading: true
                });
                mesh = new THREE.Mesh(chestGeometry, chestMaterial);
                mesh.position.y = 0.6;
                mesh.castShadow = true;
                break;
        }

        return mesh;
    }

    /**
     * Create name label above player
     */
    createNameLabel() {
        // Create canvas for text
        const canvas = document.createElement('canvas');
        const context = canvas.getContext('2d');
        canvas.width = 256;
        canvas.height = 64;

        // Draw text
        context.fillStyle = '#FFFF00';  // Yellow
        context.font = 'Bold 32px Arial';
        context.textAlign = 'center';
        context.fillText(this.username, 128, 40);

        // Create sprite
        const texture = new THREE.CanvasTexture(canvas);
        const spriteMaterial = new THREE.SpriteMaterial({ map: texture });
        const sprite = new THREE.Sprite(spriteMaterial);
        sprite.scale.set(2, 0.5, 1);
        sprite.position.y = 2.2;  // Above head

        this.nameLabel = sprite;
        this.group.add(sprite);
    }

    /**
     * Update animation
     * @param {number} delta - Time since last frame
     */
    update(delta) {
        this.animationTime += delta;

        // Simple idle animation (bob up and down)
        if (this.animationState === 'idle') {
            const bobAmount = Math.sin(this.animationTime * 2) * 0.05;
            this.group.position.y = this.position.y + bobAmount;
        }

        // Walking animation (swing arms/legs)
        else if (this.animationState === 'walk') {
            const swingAmount = Math.sin(this.animationTime * 8) * 0.3;

            if (this.bodyParts.arms && this.bodyParts.arms.left) {
                this.bodyParts.arms.left.rotation.x = swingAmount;
                this.bodyParts.arms.right.rotation.x = -swingAmount;
            }

            if (this.bodyParts.legs && this.bodyParts.legs.left) {
                this.bodyParts.legs.left.rotation.x = -swingAmount;
                this.bodyParts.legs.right.rotation.x = swingAmount;
            }
        }
    }

    /**
     * Set animation state
     */
    setAnimation(state) {
        if (this.animationState !== state) {
            this.animationState = state;
            this.animationTime = 0;
            console.log(`[PlayerModel] ${this.username} animation: ${state}`);
        }
    }

    /**
     * Move player to position
     */
    setPosition(x, y, z) {
        this.position = { x, y, z };
        this.group.position.set(x, y, z);
    }

    /**
     * Rotate player to face direction
     */
    setRotation(yaw) {
        this.group.rotation.y = yaw;
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
     * Get Three.js group for adding to scene
     */
    getGroup() {
        return this.group;
    }
}
