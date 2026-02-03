/**
 * ModelParser.js
 * Parses RuneScape Build 560 model format (.dat) and converts to Three.js geometry
 *
 * Model Format:
 * - Header: vertex count, face count, texture count
 * - Vertices: X, Y, Z coordinates (signed shorts)
 * - Faces: Triangle indices (A, B, C)
 * - Face colors: RGB565 format
 * - Texture faces (optional): UV mapping data
 * - Priorities: Rendering order
 */

import * as THREE from 'three';

export class ModelParser {
    constructor() {
        this.modelCache = new Map();
        console.log('🎨 ModelParser initialized');
    }

    /**
     * Parse a model from raw cache data
     * @param {Uint8Array} data - Raw model data from cache
     * @returns {Object} Parsed model data
     */
    parseModel(data) {
        if (!data || data.length === 0) {
            console.error('❌ Invalid model data');
            return null;
        }

        const buffer = new DataView(data.buffer, data.byteOffset, data.byteLength);
        let offset = 0;

        try {
            // Read header
            const header = this.readHeader(buffer, offset);
            offset += header.headerSize;

            console.log(`📐 Model: ${header.vertexCount} verts, ${header.faceCount} faces, ${header.texturedFaceCount} textured`);

            // Read vertex data
            const vertices = this.readVertices(buffer, offset, header.vertexCount);
            offset += header.vertexCount * 6; // 3 shorts per vertex

            // Read face indices
            const faces = this.readFaces(buffer, offset, header.faceCount);
            offset += header.faceCount * 6; // 3 shorts per face

            // Read face types/priorities
            let faceTypes = null;
            if (header.hasFaceTypes) {
                faceTypes = this.readFaceTypes(buffer, offset, header.faceCount);
                offset += header.faceCount;
            }

            // Read face priorities
            let facePriorities = null;
            if (header.hasFacePriorities) {
                facePriorities = this.readFacePriorities(buffer, offset, header.faceCount);
                offset += header.faceCount;
            }

            // Read face alphas
            let faceAlphas = null;
            if (header.hasFaceAlphas) {
                faceAlphas = this.readFaceAlphas(buffer, offset, header.faceCount);
                offset += header.faceCount;
            }

            // Read face colors
            const faceColors = this.readFaceColors(buffer, offset, header.faceCount);
            offset += header.faceCount * 2; // 1 short per face

            // Read texture coordinates (if any)
            let textureCoords = null;
            if (header.texturedFaceCount > 0) {
                textureCoords = this.readTextureCoords(buffer, offset, header.texturedFaceCount);
            }

            return {
                vertices,
                faces,
                faceColors,
                faceTypes,
                facePriorities,
                faceAlphas,
                textureCoords,
                vertexCount: header.vertexCount,
                faceCount: header.faceCount,
                texturedFaceCount: header.texturedFaceCount
            };
        } catch (error) {
            console.error('❌ Error parsing model:', error);
            return null;
        }
    }

    /**
     * Read model header
     */
    readHeader(buffer, offset) {
        const vertexCount = buffer.getUint16(offset, false); // Big-endian
        const faceCount = buffer.getUint16(offset + 2, false);
        const texturedFaceCount = buffer.getUint8(offset + 4);

        // Read flags
        const flags = buffer.getUint8(offset + 5);
        const hasFaceTypes = (flags & 0x01) !== 0;
        const hasFacePriorities = (flags & 0x02) !== 0;
        const hasFaceAlphas = (flags & 0x04) !== 0;
        const hasFaceSkins = (flags & 0x08) !== 0;
        const hasVertexSkins = (flags & 0x10) !== 0;

        return {
            vertexCount,
            faceCount,
            texturedFaceCount,
            hasFaceTypes,
            hasFacePriorities,
            hasFaceAlphas,
            hasFaceSkins,
            hasVertexSkins,
            headerSize: 6
        };
    }

    /**
     * Read vertices (X, Y, Z coordinates)
     */
    readVertices(buffer, offset, count) {
        const vertices = [];

        for (let i = 0; i < count; i++) {
            const x = buffer.getInt16(offset + i * 6, false);
            const y = buffer.getInt16(offset + i * 6 + 2, false);
            const z = buffer.getInt16(offset + i * 6 + 4, false);

            vertices.push({ x, y, z });
        }

        return vertices;
    }

    /**
     * Read face indices (triangles)
     */
    readFaces(buffer, offset, count) {
        const faces = [];

        for (let i = 0; i < count; i++) {
            const a = buffer.getUint16(offset + i * 6, false);
            const b = buffer.getUint16(offset + i * 6 + 2, false);
            const c = buffer.getUint16(offset + i * 6 + 4, false);

            faces.push({ a, b, c });
        }

        return faces;
    }

    /**
     * Read face types
     */
    readFaceTypes(buffer, offset, count) {
        const types = [];

        for (let i = 0; i < count; i++) {
            types.push(buffer.getUint8(offset + i));
        }

        return types;
    }

    /**
     * Read face priorities
     */
    readFacePriorities(buffer, offset, count) {
        const priorities = [];

        for (let i = 0; i < count; i++) {
            priorities.push(buffer.getUint8(offset + i));
        }

        return priorities;
    }

    /**
     * Read face alpha values
     */
    readFaceAlphas(buffer, offset, count) {
        const alphas = [];

        for (let i = 0; i < count; i++) {
            alphas.push(buffer.getUint8(offset + i));
        }

        return alphas;
    }

    /**
     * Read face colors (RGB565 format)
     */
    readFaceColors(buffer, offset, count) {
        const colors = [];

        for (let i = 0; i < count; i++) {
            const rgb565 = buffer.getUint16(offset + i * 2, false);
            colors.push(rgb565);
        }

        return colors;
    }

    /**
     * Read texture coordinates
     */
    readTextureCoords(buffer, offset, count) {
        const coords = [];

        for (let i = 0; i < count; i++) {
            // Texture coordinate format varies, simplified version
            const u = buffer.getInt16(offset + i * 4, false);
            const v = buffer.getInt16(offset + i * 4 + 2, false);
            coords.push({ u, v });
        }

        return coords;
    }

    /**
     * Convert RGB565 to RGB (0-1 range)
     */
    convertRGB565(rgb565) {
        const r = ((rgb565 >> 11) & 0x1f) / 31.0;
        const g = ((rgb565 >> 5) & 0x3f) / 63.0;
        const b = (rgb565 & 0x1f) / 31.0;
        return { r, g, b };
    }

    /**
     * Create Three.js geometry from parsed model data
     * @param {Object} modelData - Parsed model data
     * @returns {THREE.BufferGeometry}
     */
    createGeometry(modelData) {
        if (!modelData) {
            return null;
        }

        const geometry = new THREE.BufferGeometry();

        // Convert vertices to Float32Array
        const positions = new Float32Array(modelData.vertexCount * 3);
        for (let i = 0; i < modelData.vertexCount; i++) {
            const v = modelData.vertices[i];
            // Scale from RS units to world units (divide by 128)
            // Swap Y and Z (RS uses Y-up, Three.js uses Y-up but different axis)
            positions[i * 3] = v.x / 128;
            positions[i * 3 + 1] = -v.y / 128; // Negative Y
            positions[i * 3 + 2] = -v.z / 128; // Negative Z
        }
        geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3));

        // Convert face colors to vertex colors
        const colors = new Float32Array(modelData.faceCount * 9); // 3 vertices per face, RGB per vertex
        for (let i = 0; i < modelData.faceCount; i++) {
            const rgb = this.convertRGB565(modelData.faceColors[i]);

            // Apply color to all 3 vertices of the triangle
            for (let j = 0; j < 3; j++) {
                colors[i * 9 + j * 3] = rgb.r;
                colors[i * 9 + j * 3 + 1] = rgb.g;
                colors[i * 9 + j * 3 + 2] = rgb.b;
            }
        }
        geometry.setAttribute('color', new THREE.BufferAttribute(colors, 3));

        // Convert face indices
        const indices = new Uint16Array(modelData.faceCount * 3);
        for (let i = 0; i < modelData.faceCount; i++) {
            const face = modelData.faces[i];
            indices[i * 3] = face.a;
            indices[i * 3 + 1] = face.b;
            indices[i * 3 + 2] = face.c;
        }
        geometry.setIndex(new THREE.BufferAttribute(indices, 1));

        // Compute normals for lighting
        geometry.computeVertexNormals();

        // Compute bounding sphere for frustum culling
        geometry.computeBoundingSphere();

        return geometry;
    }

    /**
     * Create Three.js mesh from parsed model data
     * @param {Object} modelData - Parsed model data
     * @param {Object} options - Material options
     * @returns {THREE.Mesh}
     */
    createMesh(modelData, options = {}) {
        const geometry = this.createGeometry(modelData);
        if (!geometry) {
            return null;
        }

        // Create material
        const material = new THREE.MeshLambertMaterial({
            vertexColors: true,
            flatShading: options.flatShading !== false, // Default to flat shading (OSRS style)
            side: options.doubleSided ? THREE.DoubleSide : THREE.FrontSide,
            transparent: options.transparent || false,
            opacity: options.opacity || 1.0
        });

        const mesh = new THREE.Mesh(geometry, material);
        mesh.castShadow = options.castShadow !== false;
        mesh.receiveShadow = options.receiveShadow !== false;

        return mesh;
    }

    /**
     * Parse and create mesh in one step (with caching)
     * @param {Uint8Array} data - Raw model data
     * @param {number} modelId - Model ID for caching
     * @param {Object} options - Material options
     * @returns {THREE.Mesh}
     */
    parseAndCreate(data, modelId, options = {}) {
        // Check cache
        const cacheKey = `model_${modelId}`;
        if (this.modelCache.has(cacheKey)) {
            const cached = this.modelCache.get(cacheKey);
            return cached.clone();
        }

        // Parse model
        const modelData = this.parseModel(data);
        if (!modelData) {
            return null;
        }

        // Create mesh
        const mesh = this.createMesh(modelData, options);
        if (!mesh) {
            return null;
        }

        // Cache it
        this.modelCache.set(cacheKey, mesh);

        console.log(`✅ Model ${modelId} parsed and cached`);

        // Return a clone so original stays in cache
        return mesh.clone();
    }

    /**
     * Clear cached models to free memory
     */
    clearCache() {
        console.log(`🗑️ Clearing ${this.modelCache.size} cached models`);
        this.modelCache.forEach(mesh => {
            mesh.geometry.dispose();
            mesh.material.dispose();
        });
        this.modelCache.clear();
    }

    /**
     * Get cache statistics
     */
    getStats() {
        return {
            cachedModels: this.modelCache.size,
            memoryEstimate: this.estimateMemory()
        };
    }

    /**
     * Estimate memory usage
     */
    estimateMemory() {
        let bytes = 0;

        this.modelCache.forEach(mesh => {
            const geometry = mesh.geometry;
            if (geometry.attributes.position) {
                bytes += geometry.attributes.position.array.byteLength;
            }
            if (geometry.attributes.color) {
                bytes += geometry.attributes.color.array.byteLength;
            }
            if (geometry.index) {
                bytes += geometry.index.array.byteLength;
            }
        });

        return (bytes / 1024 / 1024).toFixed(2) + ' MB';
    }
}

/**
 * Common player model IDs (Build 560)
 */
export const PlayerModelIds = {
    MALE_HEAD: 0,
    FEMALE_HEAD: 1,
    MALE_BODY: 18,
    FEMALE_BODY: 36,
    MALE_ARMS: 26,
    FEMALE_ARMS: 65,
    MALE_HANDS: 33,
    FEMALE_HANDS: 67,
    MALE_LEGS: 36,
    FEMALE_LEGS: 68,
    MALE_FEET: 42,
    FEMALE_FEET: 79
};

/**
 * Common NPC model IDs (examples)
 */
export const NpcModelIds = {
    GOBLIN: 100,
    COW: 81,
    CHICKEN: 41,
    MAN: 1,
    WOMAN: 2,
    GUARD: 9,
    SHOPKEEPER: 17
};
