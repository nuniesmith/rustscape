# 🚀 Quick Start Guide - Rustscape 3D Client

**Last Updated:** Current Session  
**Status:** ✅ Ready to Use  
**Time to Start:** < 2 minutes

---

## ⚡ Fastest Path to Development

### 1. Navigate to Client Directory
```bash
cd rustscape/src/client
```

### 2. Install Dependencies (First Time Only)
```bash
npm install
```

### 3. Start Development Server
```bash
npm run dev
```

### 4. Open Your Browser
**URL:** http://localhost:3000

**That's it!** You're now running the 3D client! 🎉

---

## 📁 Project Location

You are here: `rustscape/src/client/`

```
rustscape/
└── src/
    └── client/          ← YOU ARE HERE
        ├── src/         ← Your JavaScript code
        ├── assets/      ← CSS, images, fonts
        ├── public/      ← HTML files
        └── package.json
```

---

## 🎯 What You'll See

When you open http://localhost:3000, you'll see:

- ✅ **3D Canvas** - Black screen (Three.js ready)
- ✅ **Loading Screen** - "RUSTSCAPE" logo with progress bar
- ✅ **Classic RS UI** - Brown RuneScape-themed interface
- ✅ **Sidebar** - Inventory, stats, equipment tabs
- ✅ **Chat Box** - Bottom chat interface
- ✅ **Minimap** - Top-right circular minimap
- ✅ **FPS Counter** - Top-left (shows 60 FPS)

**Current State:** Player + 4 NPCs should spawn and wander around

---

## 📝 Common Commands

### Development
```bash
npm run dev        # Start dev server (port 3000)
npm run build      # Build for production
npm run preview    # Preview production build
```

### Testing
```bash
# Check if server works
curl http://localhost:3000

# View build output
ls -lh dist/
```

---

## 📂 Where to Find Things

### Source Code
```
src/
├── main.js              ← Start here! Entry point
├── cache/               ← Cache reading system
│   ├── CacheReader.js
│   └── ModelParser.js
├── entities/            ← Game entities
│   ├── PlayerModel.js   ← Player with equipment
│   └── NPC.js           ← NPCs with AI
└── utils/
    └── constants.js     ← Game configuration
```

### Assets
```
assets/
└── styles/
    └── main.css         ← All CSS styles
```

### HTML
```
public/
└── index.html           ← Main HTML file
```

---

## 💻 How to Code

### Import with Path Aliases
```javascript
// In any .js file:
import { PlayerModel } from "@entities/PlayerModel.js";
import { CacheReader } from "@cache/CacheReader.js";
import { GAME_CONFIG } from "@utils/constants.js";
```

### Use Constants
```javascript
import { GAME_CONFIG, COLORS } from "@utils/constants.js";

console.log(GAME_CONFIG.TARGET_FPS);  // 60
console.log(COLORS.RS_GOLD);          // 0xd4a017
```

### Add New Module
```javascript
// 1. Create file: src/systems/MySystem.js
export class MySystem {
    constructor() {
        console.log("My system!");
    }
}

// 2. Import in main.js
import { MySystem } from "@/systems/MySystem.js";
```

---

## 🎨 Modify Styles

### Edit CSS
```bash
# Open the main stylesheet
nano assets/styles/main.css
```

Changes are live-reloaded automatically!

---

## 🐛 Troubleshooting

### Dev server won't start?
```bash
# Remove and reinstall dependencies
rm -rf node_modules package-lock.json
npm install
npm run dev
```

### Port 3000 already in use?
```bash
# Kill existing process
pkill -f "vite"

# Or use different port
npx vite --port 3001
```

### Module not found?
- Restart dev server
- Check import uses path alias: `@cache/` not `./cache/`
- Verify file exists in `src/`

### CSS not loading?
- Check HTML uses `/styles/main.css` (absolute path)
- Restart dev server
- Clear browser cache

---

## 📚 Documentation

**Start Here:**
1. `SETUP_VERIFIED.md` - Verification checklist
2. `STRUCTURE.md` - Detailed structure
3. `README.md` - Client overview

**Need Help?**
- `MIGRATION_GUIDE.md` - How to update code
- `REORGANIZATION_SUMMARY.md` - What changed
- `../PROJECT_OVERVIEW.md` - Full project status
- `../RESEARCH_GUIDE.md` - Learning resources

---

## 🎯 What to Do Next

### Immediate (Today)
1. ✅ Run `npm run dev` - Verify it works
2. 📖 Read `STRUCTURE.md` - Understand organization
3. 🔍 Explore `src/main.js` - See how it works
4. 📝 Check `src/utils/constants.js` - See available config

### Short-term (This Week)
5. Wire up real cache models (replace placeholders)
6. Implement skeletal animation system
7. Add click-to-walk interaction

### Reference
- See `../NEXT_STEPS.md` for detailed roadmap
- See `../RESEARCH_GUIDE.md` for learning topics

---

## 🎊 You're Ready!

The setup is complete and verified. Start the dev server and begin coding!

```bash
cd rustscape/src/client
npm run dev
```

**Happy coding! 🚀**

---

## 🔗 Quick Reference

| Task | Command |
|------|---------|
| Start dev | `npm run dev` |
| Build | `npm run build` |
| Preview | `npm run preview` |
| Clean install | `rm -rf node_modules && npm install` |
| Check imports | `grep -r "^import" src/` |

| File | Purpose |
|------|---------|
| `src/main.js` | Entry point |
| `src/utils/constants.js` | Configuration |
| `assets/styles/main.css` | Styles |
| `public/index.html` | HTML |
| `vite.config.js` | Build config |

| URL | Purpose |
|-----|---------|
| http://localhost:3000 | Dev server |
| http://localhost:4173 | Preview build |
| http://localhost:8080 | Rust server (when running) |

---

**Everything is ready. Just run `npm run dev` and start building!** ✨