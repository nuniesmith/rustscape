# 🔄 Migration Guide - Client Reorganization

**Date:** Current Session  
**Purpose:** Guide for migrating to the new client structure  
**Impact:** All import paths have changed

---

## 📋 Overview

The client has been reorganized for better maintainability:

- **Source code** moved to `src/`
- **Assets** (CSS, images) moved to `assets/`
- **HTML** moved to `public/`
- **Path aliases** added for cleaner imports

---

## 🗺️ Directory Migration Map

### Old Structure → New Structure

```
OLD                                 NEW
─────────────────────────────────────────────────────────────
dist/3d/index.html                → public/index.html
dist/3d/js/main.js                → src/main.js
dist/3d/js/cache/CacheReader.js   → src/cache/CacheReader.js
dist/3d/js/cache/ModelParser.js   → src/cache/ModelParser.js
dist/3d/js/entities/PlayerModel.js → src/entities/PlayerModel.js
dist/3d/js/entities/NPC.js        → src/entities/NPC.js
dist/3d/js/rendering/             → src/rendering/
dist/3d/css/                      → assets/styles/
(inline styles in HTML)           → assets/styles/main.css
```

---

## 🔧 Code Changes Required

### 1. Update Import Paths

**Before (Old):**
```javascript
import { PlayerModel } from "./entities/PlayerModel.js";
import { CacheReader } from "./cache/CacheReader.js";
import { ModelParser } from "./cache/ModelParser.js";
```

**After (New):**
```javascript
import { PlayerModel } from "@entities/PlayerModel.js";
import { CacheReader } from "@cache/CacheReader.js";
import { ModelParser } from "@cache/ModelParser.js";
import { GAME_CONFIG } from "@utils/constants.js";
```

### 2. Update Vite Config References

**Before:**
```javascript
root: 'dist/3d'
```

**After:**
```javascript
root: 'public'
```

### 3. Extract Magic Numbers to Constants

**Before:**
```javascript
const distance = 20;  // What is this?
const cameraAngle = 26.565;  // Why this value?
```

**After:**
```javascript
import { GAME_CONFIG } from "@utils/constants.js";

const distance = GAME_CONFIG.CAMERA_DISTANCE;
const cameraAngle = GAME_CONFIG.CAMERA_PITCH;
```

---

## 📝 Step-by-Step Migration

### Step 1: Update Your Local Repository

```bash
# Pull latest changes
cd rustscape/client
git pull origin main

# Install dependencies (if needed)
npm install
```

### Step 2: Update Your Imports

Search for old import patterns and replace:

```bash
# Find all relative imports
grep -r "from \"\./" src/

# Replace with path aliases manually
# Example: "./cache/CacheReader.js" → "@cache/CacheReader.js"
```

### Step 3: Update Configuration References

If you have custom configuration:

```javascript
// OLD: Hardcoded values
const serverUrl = "localhost:8080";

// NEW: Use constants
import { GAME_CONFIG } from "@utils/constants.js";
const serverUrl = `${GAME_CONFIG.SERVER_URL}:${GAME_CONFIG.SERVER_PORT}`;
```

### Step 4: Test Your Changes

```bash
# Start dev server
npm run dev

# Check browser console for errors
# Visit http://localhost:3000
```

---

## 🔍 Common Migration Issues

### Issue 1: "Module not found"

**Error:**
```
Failed to resolve import "./cache/CacheReader.js"
```

**Solution:**
Update to use path alias:
```javascript
import { CacheReader } from "@cache/CacheReader.js";
```

### Issue 2: "Cannot find module '@cache'"

**Error:**
```
Cannot resolve '@cache/CacheReader.js'
```

**Solution:**
Make sure you're using the new `vite.config.js`:
- Check that `resolve.alias` has path aliases defined
- Restart dev server: `npm run dev`

### Issue 3: CSS not loading

**Error:**
Styles missing from page

**Solution:**
Update HTML link:
```html
<!-- OLD -->
<style>...</style>

<!-- NEW -->
<link rel="stylesheet" href="../assets/styles/main.css">
```

### Issue 4: Build fails

**Error:**
```
Cannot resolve entry point
```

**Solution:**
Update `vite.config.js`:
```javascript
{
  root: "public",  // Not "dist/3d"
  publicDir: "../assets"
}
```

---

## 📦 Available Path Aliases

```javascript
@         → src/
@cache    → src/cache/
@entities → src/entities/
@rendering → src/rendering/
@utils    → src/utils/
@assets   → assets/
```

### Usage Examples

```javascript
// Main source
import { something } from "@/main.js";

// Cache
import { CacheReader } from "@cache/CacheReader.js";

// Entities
import { PlayerModel } from "@entities/PlayerModel.js";
import { NPC } from "@entities/NPC.js";

// Utils
import { GAME_CONFIG, COLORS } from "@utils/constants.js";

// Assets (images, etc.)
import logo from "@assets/images/logo.png";
```

---

## 🎯 Benefits of New Structure

### Before
❌ 621-line HTML file with inline CSS  
❌ Relative import paths: `../../../cache/CacheReader.js`  
❌ Magic numbers scattered everywhere  
❌ Hard to find files  
❌ Mixing concerns (HTML + CSS + JS)

### After
✅ Clean 155-line HTML  
✅ Path aliases: `@cache/CacheReader.js`  
✅ Centralized constants  
✅ Clear directory structure  
✅ Proper separation of concerns

---

## 📚 New Files Added

### `src/utils/constants.js`
All game configuration in one place:
- `GAME_CONFIG` - Game settings
- `CACHE_CONFIG` - Cache paths and IDs
- `UI_CONFIG` - UI dimensions
- `COLORS` - RuneScape theme colors
- `PacketType` - Network packets
- Helper functions (XP, combat level)

### `assets/styles/main.css`
Complete stylesheet extracted from HTML:
- RuneScape Classic theme
- All UI components
- Responsive design
- Well-organized sections

### `public/index.html`
Clean HTML structure:
- No inline styles
- External CSS link
- Module script
- Semantic markup

---

## 🧪 Testing Checklist

After migration, verify:

- [ ] Dev server starts: `npm run dev`
- [ ] Page loads at http://localhost:3000
- [ ] Styles applied correctly
- [ ] Three.js canvas renders
- [ ] Player model loads
- [ ] NPCs spawn
- [ ] Equipment system works
- [ ] Chat box functional
- [ ] No console errors
- [ ] Build succeeds: `npm run build`

---

## 💡 Migration Tips

### Tip 1: Use Find & Replace
```bash
# VS Code: Ctrl+Shift+H (Windows) or Cmd+Shift+H (Mac)
Find:    from "./cache/
Replace: from "@cache/
```

### Tip 2: Check All Imports
```bash
# List all imports in src/
grep -r "^import" src/
```

### Tip 3: Update One File at a Time
Start with `main.js`, then work through dependencies

### Tip 4: Use TypeScript Definitions (Future)
Add JSDoc comments for better IDE support:
```javascript
/**
 * @typedef {import('@utils/constants.js').GAME_CONFIG} GameConfig
 */
```

---

## 🔄 Rollback Instructions

If you need to rollback:

```bash
# Revert to old structure
git checkout HEAD~1 client/

# Reinstall dependencies
cd client
npm install

# Start old dev server
npm run dev
```

**Note:** Old structure used `root: 'dist/3d'` in vite.config.js

---

## 📞 Getting Help

### If You're Stuck

1. **Check this guide** - Most common issues covered
2. **Review STRUCTURE.md** - Detailed structure documentation
3. **Check git diff** - See what changed
4. **Ask for help** - Include error messages

### Useful Commands

```bash
# Check git changes
git diff

# See file history
git log --oneline client/

# Check current structure
tree -L 3 client/

# Verify imports
npm run build
```

---

## 🎉 Post-Migration

Once migrated successfully:

1. **Delete old directories:**
   - `dist/3d/js/` (now `src/`)
   - `dist/3d/css/` (now `assets/styles/`)

2. **Update documentation:**
   - Update any custom docs
   - Update team wiki

3. **Share with team:**
   - Send this guide
   - Answer questions

---

## 📊 File Count Comparison

### Before
```
dist/3d/
├── index.html (621 lines - HTML+CSS)
└── js/
    ├── main.js
    ├── cache/ (2 files)
    └── entities/ (2 files)

Total: 6 files
```

### After
```
src/
├── main.js
├── cache/ (2 files)
├── entities/ (2 files)
└── utils/constants.js

assets/styles/main.css

public/index.html (155 lines - HTML only)

Total: 8 files (better organized!)
```

---

## ✅ Migration Complete!

You should now have:
- ✅ Clean, organized directory structure
- ✅ Path aliases working
- ✅ Centralized constants
- ✅ Separated CSS from HTML
- ✅ Better maintainability

**Next steps:**
- Read `STRUCTURE.md` for detailed structure info
- Review `src/utils/constants.js` for available constants
- Continue development with cleaner code!

---

**Questions?** Check STRUCTURE.md or PROJECT_OVERVIEW.md