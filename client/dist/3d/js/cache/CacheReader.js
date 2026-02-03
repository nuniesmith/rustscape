/**
 * CacheReader.js
 * Reads and parses RuneScape Build 560 cache data
 *
 * Cache Structure:
 * - main_file_cache.dat2 - Contains all actual data
 * - main_file_cache.idx0-28 - Index files (pointers into dat2)
 *
 * Each archive contains multiple groups (files)
 * Groups contain actual game data (models, textures, sprites, etc.)
 */

export class CacheReader {
    constructor(cacheBaseUrl = '/assets/data_caches/560') {
        this.baseUrl = cacheBaseUrl;
        this.datFile = null;
        this.indexFiles = new Map();
        this.archives = new Map();
        this.loadingPromises = new Map();

        console.log('📦 CacheReader initialized with base:', this.baseUrl);
    }

    /**
     * Load the main data file (dat2)
     * This contains all the actual cache data
     */
    async loadDataFile() {
        if (this.datFile) {
            return this.datFile;
        }

        console.log('📥 Loading main_file_cache.dat2...');

        try {
            const response = await fetch(`${this.baseUrl}/main_file_cache.dat2`);
            if (!response.ok) {
                throw new Error(`Failed to load dat2: ${response.status}`);
            }

            const arrayBuffer = await response.arrayBuffer();
            this.datFile = new DataView(arrayBuffer);

            console.log(`✅ Loaded dat2: ${(arrayBuffer.byteLength / 1024 / 1024).toFixed(2)} MB`);
            return this.datFile;
        } catch (error) {
            console.error('❌ Failed to load dat2:', error);
            throw error;
        }
    }

    /**
     * Load an index file
     * @param {number} archiveId - Archive ID (0-28)
     */
    async loadIndexFile(archiveId) {
        if (this.indexFiles.has(archiveId)) {
            return this.indexFiles.get(archiveId);
        }

        // Check if already loading
        if (this.loadingPromises.has(`idx${archiveId}`)) {
            return this.loadingPromises.get(`idx${archiveId}`);
        }

        const loadPromise = (async () => {
            console.log(`📥 Loading main_file_cache.idx${archiveId}...`);

            try {
                const response = await fetch(`${this.baseUrl}/main_file_cache.idx${archiveId}`);
                if (!response.ok) {
                    throw new Error(`Failed to load idx${archiveId}: ${response.status}`);
                }

                const arrayBuffer = await response.arrayBuffer();
                const indexData = new DataView(arrayBuffer);

                this.indexFiles.set(archiveId, indexData);
                console.log(`✅ Loaded idx${archiveId}: ${(arrayBuffer.byteLength / 1024).toFixed(2)} KB`);

                return indexData;
            } catch (error) {
                console.error(`❌ Failed to load idx${archiveId}:`, error);
                throw error;
            } finally {
                this.loadingPromises.delete(`idx${archiveId}`);
            }
        })();

        this.loadingPromises.set(`idx${archiveId}`, loadPromise);
        return loadPromise;
    }

    /**
     * Load an entire archive
     * @param {number} archiveId - Archive ID (0-28)
     */
    async loadArchive(archiveId) {
        if (this.archives.has(archiveId)) {
            return this.archives.get(archiveId);
        }

        console.log(`📦 Loading archive ${archiveId}...`);

        // Load both index and data file
        const [indexData, datFile] = await Promise.all([
            this.loadIndexFile(archiveId),
            this.loadDataFile()
        ]);

        const archive = new Archive(archiveId, indexData, datFile);
        this.archives.set(archiveId, archive);

        console.log(`✅ Archive ${archiveId} ready: ${archive.getGroupCount()} groups`);
        return archive;
    }

    /**
     * Get a specific file (group) from an archive
     * @param {number} archiveId - Archive ID
     * @param {number} groupId - Group (file) ID within the archive
     */
    async getFile(archiveId, groupId) {
        const archive = await this.loadArchive(archiveId);
        return archive.getGroup(groupId);
    }

    /**
     * Get a model from archive 7
     * @param {number} modelId - Model ID
     */
    async getModel(modelId) {
        console.log(`🎨 Loading model ${modelId}...`);
        return this.getFile(7, modelId);
    }

    /**
     * Get a texture from archive 9
     * @param {number} textureId - Texture ID
     */
    async getTexture(textureId) {
        console.log(`🖼️ Loading texture ${textureId}...`);
        return this.getFile(9, textureId);
    }

    /**
     * Get a sprite from archive 8
     * @param {number} spriteId - Sprite ID
     */
    async getSprite(spriteId) {
        console.log(`🎴 Loading sprite ${spriteId}...`);
        return this.getFile(8, spriteId);
    }

    /**
     * Get map data from archive 5
     * @param {number} regionX - Region X coordinate
     * @param {number} regionY - Region Y coordinate
     */
    async getMapData(regionX, regionY) {
        const mapId = regionX * 256 + regionY;
        console.log(`🗺️ Loading map region (${regionX}, ${regionY})...`);
        return this.getFile(5, mapId);
    }

    /**
     * Clear all cached data to free memory
     */
    clearCache() {
        console.log('🗑️ Clearing cache...');
        this.archives.clear();
        this.indexFiles.clear();
        this.datFile = null;
    }

    /**
     * Get cache statistics
     */
    getStats() {
        return {
            archivesLoaded: this.archives.size,
            indexFilesLoaded: this.indexFiles.size,
            datFileLoaded: this.datFile !== null,
            memoryUsage: this.estimateMemoryUsage()
        };
    }

    /**
     * Estimate memory usage in MB
     */
    estimateMemoryUsage() {
        let bytes = 0;

        if (this.datFile) {
            bytes += this.datFile.byteLength;
        }

        this.indexFiles.forEach(indexData => {
            bytes += indexData.byteLength;
        });

        return (bytes / 1024 / 1024).toFixed(2) + ' MB';
    }
}

/**
 * Archive class
 * Represents a single archive (idx file + dat2 section)
 */
class Archive {
    constructor(archiveId, indexData, datFile) {
        this.archiveId = archiveId;
        this.indexData = indexData;
        this.datFile = datFile;
        this.groups = new Map();

        this.parseIndex();
    }

    /**
     * Parse the index file to get group locations
     */
    parseIndex() {
        const indexSize = this.indexData.byteLength;
        const entrySize = 6; // Each index entry is 6 bytes
        this.groupCount = Math.floor(indexSize / entrySize);

        console.log(`📋 Archive ${this.archiveId}: ${this.groupCount} entries`);
    }

    /**
     * Get a group (file) from this archive
     * @param {number} groupId - Group ID
     */
    getGroup(groupId) {
        // Check cache
        if (this.groups.has(groupId)) {
            return this.groups.get(groupId);
        }

        // Calculate index position
        const indexPosition = groupId * 6;

        if (indexPosition + 6 > this.indexData.byteLength) {
            console.warn(`⚠️ Group ${groupId} out of bounds in archive ${this.archiveId}`);
            return null;
        }

        // Read index entry (6 bytes)
        // Format: [size:3bytes][sector:3bytes]
        const size = (this.indexData.getUint8(indexPosition) << 16) |
                     (this.indexData.getUint8(indexPosition + 1) << 8) |
                     this.indexData.getUint8(indexPosition + 2);

        const sector = (this.indexData.getUint8(indexPosition + 3) << 16) |
                       (this.indexData.getUint8(indexPosition + 4) << 8) |
                       this.indexData.getUint8(indexPosition + 5);

        if (size === 0 || sector === 0) {
            console.warn(`⚠️ Invalid index entry for group ${groupId} in archive ${this.archiveId}`);
            return null;
        }

        // Read data from dat2 file
        const data = this.readGroupData(groupId, sector, size);

        if (data) {
            this.groups.set(groupId, data);
        }

        return data;
    }

    /**
     * Read group data from dat2 file
     * @param {number} groupId - Group ID
     * @param {number} sector - Starting sector in dat2
     * @param {number} size - Data size
     */
    readGroupData(groupId, sector, size) {
        const sectorSize = 520; // Standard RS cache sector size
        const headerSize = 8; // Each sector has 8-byte header

        const buffer = new Uint8Array(size);
        let bufferOffset = 0;
        let currentSector = sector;
        let remainingSize = size;

        try {
            while (remainingSize > 0) {
                const sectorPosition = currentSector * sectorSize;

                if (sectorPosition + sectorSize > this.datFile.byteLength) {
                    console.error(`❌ Sector ${currentSector} out of bounds`);
                    return null;
                }

                // Read sector header (8 bytes)
                const headerId = this.datFile.getUint16(sectorPosition);
                const headerChunk = this.datFile.getUint16(sectorPosition + 2);
                const headerNext = (this.datFile.getUint8(sectorPosition + 4) << 16) |
                                   (this.datFile.getUint8(sectorPosition + 5) << 8) |
                                   this.datFile.getUint8(sectorPosition + 6);
                const headerArchive = this.datFile.getUint8(sectorPosition + 7);

                // Validate header
                if (headerId !== groupId || headerArchive !== this.archiveId) {
                    console.error(`❌ Invalid sector header at ${currentSector}`);
                    return null;
                }

                // Read data from this sector
                const dataSize = Math.min(remainingSize, sectorSize - headerSize);

                for (let i = 0; i < dataSize; i++) {
                    buffer[bufferOffset++] = this.datFile.getUint8(sectorPosition + headerSize + i);
                }

                remainingSize -= dataSize;
                currentSector = headerNext;

                // Safety check: prevent infinite loops
                if (remainingSize > 0 && currentSector === 0) {
                    console.error(`❌ Unexpected end of sector chain`);
                    return null;
                }
            }

            return buffer;
        } catch (error) {
            console.error(`❌ Error reading group ${groupId} from archive ${this.archiveId}:`, error);
            return null;
        }
    }

    /**
     * Get number of groups in this archive
     */
    getGroupCount() {
        return this.groupCount;
    }

    /**
     * Check if a group exists
     */
    hasGroup(groupId) {
        const indexPosition = groupId * 6;

        if (indexPosition + 6 > this.indexData.byteLength) {
            return false;
        }

        const size = (this.indexData.getUint8(indexPosition) << 16) |
                     (this.indexData.getUint8(indexPosition + 1) << 8) |
                     this.indexData.getUint8(indexPosition + 2);

        return size > 0;
    }
}

/**
 * Archive type constants
 */
export const ArchiveType = {
    ANIMATIONS: 0,
    SKELETONS: 1,
    CONFIGS: 2,
    INTERFACES: 3,
    SOUND_EFFECTS: 4,
    MAPS: 5,
    MUSIC: 6,
    MODELS: 7,
    SPRITES: 8,
    TEXTURES: 9,
    BINARY: 10,
    JINGLES: 11,
    SCRIPTS: 12,
    FONTS: 13
};

/**
 * Get archive name
 */
export function getArchiveName(archiveId) {
    const names = {
        0: 'Animations',
        1: 'Skeletons',
        2: 'Configs',
        3: 'Interfaces',
        4: 'Sound Effects',
        5: 'Maps',
        6: 'Music',
        7: 'Models',
        8: 'Sprites',
        9: 'Textures',
        10: 'Binary',
        11: 'Jingles',
        12: 'Scripts',
        13: 'Fonts'
    };
    return names[archiveId] || `Archive ${archiveId}`;
}
