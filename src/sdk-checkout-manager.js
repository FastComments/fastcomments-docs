const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const CONFIG_PATH = path.join(__dirname, 'content', 'sdk-repos.json');
const CACHE_FILE = 'sdk-cache.json';

/**
 * @typedef {Object} SDKConfig
 * @property {string} id
 * @property {string} name
 * @property {string} pageHeader
 * @property {string} repo
 * @property {string} branch
 * @property {string} language
 * @property {string} icon
 * @property {string} docGenerator
 * @property {string} description
 */

/**
 * @typedef {Object} SDKReposConfig
 * @property {Array<SDKConfig>} sdks
 * @property {string} checkoutDirectory
 * @property {string} guidesDirectory
 */

/**
 * @typedef {Object} SDKCacheEntry
 * @property {string} commitHash
 * @property {number} lastUpdated
 */

class SDKCheckoutManager {
    constructor() {
        this.config = this.loadConfig();
        this.checkoutDir = path.join(__dirname, '..', this.config.checkoutDirectory);
        this.cacheFilePath = path.join(this.checkoutDir, CACHE_FILE);
        this.cache = this.loadCache();
    }

    /**
     * Load SDK repos configuration
     * @returns {SDKReposConfig}
     */
    loadConfig() {
        if (!fs.existsSync(CONFIG_PATH)) {
            throw new Error(`SDK repos config not found at ${CONFIG_PATH}`);
        }
        return JSON.parse(fs.readFileSync(CONFIG_PATH, 'utf8'));
    }

    /**
     * Load cache of SDK checkout metadata
     * @returns {Object.<string, SDKCacheEntry>}
     */
    loadCache() {
        if (!fs.existsSync(this.cacheFilePath)) {
            return {};
        }
        try {
            return JSON.parse(fs.readFileSync(this.cacheFilePath, 'utf8'));
        } catch (e) {
            console.warn('Failed to load SDK cache, starting fresh:', e.message);
            return {};
        }
    }

    /**
     * Save cache to disk
     */
    saveCache() {
        fs.writeFileSync(this.cacheFilePath, JSON.stringify(this.cache, null, 2), 'utf8');
    }

    /**
     * Ensure checkout directory exists
     */
    ensureCheckoutDir() {
        if (!fs.existsSync(this.checkoutDir)) {
            fs.mkdirSync(this.checkoutDir, { recursive: true });
            console.log(`Created SDK checkout directory: ${this.checkoutDir}`);
        }
    }

    /**
     * Get the current commit hash of a repo
     * @param {string} repoPath - Path to the git repository
     * @returns {string|null} - Commit hash or null if not a git repo
     */
    getCommitHash(repoPath) {
        try {
            return execSync('git rev-parse HEAD', {
                cwd: repoPath,
                encoding: 'utf8'
            }).trim();
        } catch (e) {
            return null;
        }
    }

    /**
     * Get the remote commit hash for a branch
     * @param {string} repoUrl - Repository URL
     * @param {string} branch - Branch name
     * @returns {string|null} - Commit hash or null on error
     */
    getRemoteCommitHash(repoUrl, branch) {
        try {
            const output = execSync(`git ls-remote ${repoUrl} ${branch}`, {
                encoding: 'utf8'
            });
            const match = output.match(/^([a-f0-9]+)/);
            return match ? match[1] : null;
        } catch (e) {
            console.error(`Failed to get remote commit hash for ${repoUrl}:`, e.message);
            return null;
        }
    }

    /**
     * Clone a repository
     * @param {SDKConfig} sdk - SDK configuration
     * @param {string} targetPath - Target directory path
     */
    cloneRepo(sdk, targetPath) {
        console.log(`Cloning ${sdk.name} from ${sdk.repo}...`);
        try {
            execSync(`git clone --depth 1 --branch ${sdk.branch} ${sdk.repo} ${targetPath}`, {
                stdio: 'inherit'
            });
            console.log(`Successfully cloned ${sdk.name}`);
        } catch (e) {
            throw new Error(`Failed to clone ${sdk.name}: ${e.message}`);
        }
    }

    /**
     * Update an existing repository
     * @param {string} repoPath - Path to the repository
     * @param {string} branch - Branch name
     */
    updateRepo(repoPath, branch) {
        console.log(`Updating repository at ${repoPath}...`);
        try {
            execSync(`git fetch origin ${branch} && git reset --hard origin/${branch}`, {
                cwd: repoPath,
                stdio: 'inherit'
            });
            console.log(`Successfully updated repository`);
        } catch (e) {
            throw new Error(`Failed to update repository: ${e.message}`);
        }
    }

    /**
     * Check if an SDK needs to be updated
     * @param {SDKConfig} sdk - SDK configuration
     * @param {string} repoPath - Local repository path
     * @returns {boolean} - True if update is needed
     */
    needsUpdate(sdk, repoPath) {
        // If repo doesn't exist locally, it needs to be cloned
        if (!fs.existsSync(repoPath)) {
            return true;
        }

        // Get local commit hash
        const localHash = this.getCommitHash(repoPath);
        if (!localHash) {
            console.warn(`${sdk.id}: No local commit hash found, will re-clone`);
            return true;
        }

        // Check cache
        const cached = this.cache[sdk.id];
        if (cached && cached.commitHash === localHash) {
            // If we have a recent cache entry, skip remote check
            const age = Date.now() - cached.lastUpdated;
            const maxAge = 1000 * 60 * 2; // 2 minutes
            if (age < maxAge) {
                console.log(`${sdk.id}: Using cached version (${localHash.substring(0, 7)})`);
                return false;
            }
        }

        // Get remote commit hash
        const remoteHash = this.getRemoteCommitHash(sdk.repo, sdk.branch);
        if (!remoteHash) {
            console.warn(`${sdk.id}: Could not get remote hash, keeping local version`);
            return false;
        }

        const needsUpdate = localHash !== remoteHash;
        if (needsUpdate) {
            console.log(`${sdk.id}: Update available (${localHash.substring(0, 7)} -> ${remoteHash.substring(0, 7)})`);
        } else {
            console.log(`${sdk.id}: Up to date (${localHash.substring(0, 7)})`);
        }

        return needsUpdate;
    }

    /**
     * Checkout or update a single SDK
     * @param {SDKConfig} sdk - SDK configuration
     * @returns {string} - Path to the checked out repository
     */
    checkoutSDK(sdk) {
        const repoPath = path.join(this.checkoutDir, sdk.id);

        if (this.needsUpdate(sdk, repoPath)) {
            // Remove existing directory if it exists and is invalid
            if (fs.existsSync(repoPath)) {
                console.log(`Removing existing checkout of ${sdk.id}...`);
                fs.rmSync(repoPath, { recursive: true, force: true });
            }

            // Clone the repository
            this.cloneRepo(sdk, repoPath);
        }

        // Update cache
        const commitHash = this.getCommitHash(repoPath);
        if (commitHash) {
            this.cache[sdk.id] = {
                commitHash,
                lastUpdated: Date.now()
            };
            this.saveCache();
        }

        return repoPath;
    }

    /**
     * Checkout all configured SDKs
     * @returns {Array<{sdk: SDKConfig, path: string}>}
     */
    checkoutAll() {
        console.log('Starting SDK checkout process...');
        this.ensureCheckoutDir();

        const results = [];

        for (const sdk of this.config.sdks) {
            try {
                const repoPath = this.checkoutSDK(sdk);
                results.push({ sdk, path: repoPath });
            } catch (e) {
                console.error(`Failed to checkout ${sdk.id}:`, e.message);
                // Continue with other SDKs even if one fails
            }
        }

        console.log(`SDK checkout complete. ${results.length}/${this.config.sdks.length} SDKs ready.`);
        return results;
    }

    /**
     * Get all SDK configurations
     * @returns {Array<SDKConfig>}
     */
    getSDKs() {
        return this.config.sdks;
    }

    /**
     * Get the path for a specific SDK
     * @param {string} sdkId - SDK identifier
     * @returns {string|null} - Path to SDK or null if not found
     */
    getSDKPath(sdkId) {
        const sdk = this.config.sdks.find(s => s.id === sdkId);
        if (!sdk) {
            return null;
        }
        return path.join(this.checkoutDir, sdkId);
    }
}

module.exports = SDKCheckoutManager;
