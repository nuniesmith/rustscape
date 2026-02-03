#!/bin/bash
# diagnose.sh - Check Rustscape deployment status
# Run from project root: ./scripts/diagnose.sh

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

echo "========================================"
echo "   Rustscape Deployment Diagnostic"
echo "   Simplified Architecture"
echo "========================================"
echo ""

# Track overall status
ISSUES=0
WARNINGS=0

check() {
    if [ $? -eq 0 ]; then
        echo -e "  ${GREEN}✓${NC} $1"
    else
        echo -e "  ${RED}✗${NC} $1"
        ISSUES=$((ISSUES + 1))
    fi
}

warn() {
    echo -e "  ${YELLOW}⚠${NC} $1"
    WARNINGS=$((WARNINGS + 1))
}

info() {
    echo -e "  ${BLUE}ℹ${NC} $1"
}

section() {
    echo ""
    echo -e "${BLUE}[$1]${NC}"
}

# === PREREQUISITES ===
section "Prerequisites"

command -v cargo &> /dev/null
check "Rust/Cargo installed"

if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version | awk '{print $2}')
    info "Rust version: $RUST_VERSION"
fi

# === PROJECT STRUCTURE ===
section "Project Structure"

if [ -d "src" ]; then
    check "src/ directory exists"
else
    echo -e "  ${RED}✗${NC} src/ directory missing"
    ISSUES=$((ISSUES + 1))
fi

if [ -f "src/Cargo.toml" ]; then
    check "Cargo.toml exists"
else
    echo -e "  ${RED}✗${NC} src/Cargo.toml missing"
    ISSUES=$((ISSUES + 1))
fi

if [ -f "src/src/main.rs" ]; then
    check "main.rs exists"
else
    echo -e "  ${RED}✗${NC} src/src/main.rs missing"
    ISSUES=$((ISSUES + 1))
fi

# === SERVER MODULES ===
section "Server Modules"

if [ -f "src/src/game/mod.rs" ]; then
    check "game/mod.rs exists"
else
    echo -e "  ${RED}✗${NC} game/mod.rs missing"
    ISSUES=$((ISSUES + 1))
fi

if [ -f "src/src/net/mod.rs" ]; then
    check "net/mod.rs exists"
else
    echo -e "  ${RED}✗${NC} net/mod.rs missing"
    ISSUES=$((ISSUES + 1))
fi

if [ -f "src/src/world/mod.rs" ]; then
    check "world/mod.rs exists"
else
    echo -e "  ${RED}✗${NC} world/mod.rs missing"
    ISSUES=$((ISSUES + 1))
fi

# === BUILD STATUS ===
section "Build Status"

if [ -f "src/target/release/rustscape" ]; then
    BINARY_SIZE=$(du -h src/target/release/rustscape 2>/dev/null | cut -f1)
    check "Server binary built (release) - $BINARY_SIZE"
elif [ -f "src/target/debug/rustscape" ]; then
    warn "Server binary built (debug only) - run: cargo build --release"
else
    echo -e "  ${RED}✗${NC} Server not built - run: cd src && cargo build"
    ISSUES=$((ISSUES + 1))
fi

# Check if server compiles
if [ -d "src" ]; then
    echo -n "  Checking if server compiles... "
    cd src
    if cargo check --quiet 2>/dev/null; then
        echo -e "${GREEN}✓${NC} Server compiles successfully"
    else
        echo -e "${RED}✗${NC} Server has compilation errors"
        ISSUES=$((ISSUES + 1))
    fi
    cd ..
fi

# === ASSETS ===
section "Game Assets"

if [ -d "src/assets/definitions" ]; then
    ITEMS_FILE="src/assets/definitions/items.json"
    NPCS_FILE="src/assets/definitions/npcs.json"

    if [ -f "$ITEMS_FILE" ]; then
        ITEMS_COUNT=$(grep -c '"id"' "$ITEMS_FILE" 2>/dev/null || echo "0")
        check "items.json exists ($ITEMS_COUNT items)"
    else
        echo -e "  ${RED}✗${NC} items.json missing"
        ISSUES=$((ISSUES + 1))
    fi

    if [ -f "$NPCS_FILE" ]; then
        NPCS_COUNT=$(grep -c '"id"' "$NPCS_FILE" 2>/dev/null || echo "0")
        check "npcs.json exists ($NPCS_COUNT NPCs)"
    else
        echo -e "  ${RED}✗${NC} npcs.json missing"
        ISSUES=$((ISSUES + 1))
    fi
else
    echo -e "  ${RED}✗${NC} assets/definitions directory missing"
    ISSUES=$((ISSUES + 1))
fi

if [ -d "src/assets/spawns" ]; then
    SPAWN_FILES=$(find src/assets/spawns -name "*.json" 2>/dev/null | wc -l)
    if [ "$SPAWN_FILES" -gt 0 ]; then
        check "Spawn files exist ($SPAWN_FILES files)"
    else
        warn "No spawn files found in assets/spawns"
    fi
else
    warn "assets/spawns directory missing"
fi

# === DATA DIRECTORY ===
section "Data & Persistence"

if [ -d "src/data" ]; then
    check "data/ directory exists"
else
    echo -e "  ${RED}✗${NC} data/ directory missing - create with: mkdir -p src/data/players"
    ISSUES=$((ISSUES + 1))
fi

if [ -d "src/data/players" ]; then
    PLAYER_COUNT=$(find src/data/players -name "*.json" 2>/dev/null | wc -l)
    check "data/players/ directory exists ($PLAYER_COUNT saved players)"
else
    warn "data/players/ directory missing - create with: mkdir -p src/data/players"
fi

# === CLIENT ===
section "Client"

if [ -d "src/client/dist" ]; then
    check "client/dist/ directory exists"

    if [ -f "src/client/dist/index.html" ]; then
        CLIENT_SIZE=$(du -h src/client/dist/index.html 2>/dev/null | cut -f1)
        check "client/dist/index.html exists ($CLIENT_SIZE)"
    else
        echo -e "  ${RED}✗${NC} client/dist/index.html missing"
        ISSUES=$((ISSUES + 1))
    fi
else
    echo -e "  ${RED}✗${NC} client/dist/ directory missing"
    ISSUES=$((ISSUES + 1))
fi

# === NETWORK ===
section "Network"

# Check if port 8080 is available
if command -v lsof &> /dev/null; then
    if lsof -i :8080 &> /dev/null; then
        PROCESS=$(lsof -i :8080 | tail -n 1 | awk '{print $1}')
        warn "Port 8080 already in use by: $PROCESS"
    else
        check "Port 8080 available"
    fi
elif command -v ss &> /dev/null; then
    if ss -tln | grep -q ":8080 "; then
        warn "Port 8080 already in use"
    else
        check "Port 8080 available"
    fi
elif command -v netstat &> /dev/null; then
    if netstat -tln | grep -q ":8080 "; then
        warn "Port 8080 already in use"
    else
        check "Port 8080 available"
    fi
else
    info "Cannot check port 8080 (no lsof/ss/netstat)"
fi

# Check Tailscale (optional but recommended)
if command -v tailscale &> /dev/null; then
    TAILSCALE_IP=$(tailscale ip -4 2>/dev/null || echo "")
    if [ -n "$TAILSCALE_IP" ]; then
        check "Tailscale connected: $TAILSCALE_IP"
        info "Friends can connect at: http://$TAILSCALE_IP:8080"
    else
        warn "Tailscale installed but not connected"
    fi
else
    info "Tailscale not installed (optional - for playing with friends)"
fi

# === DOCUMENTATION ===
section "Documentation"

if [ -f "docs/PROJECT_CONTEXT.md" ]; then
    check "PROJECT_CONTEXT.md exists"
else
    warn "PROJECT_CONTEXT.md missing"
fi

if [ -f "docs/QUICKSTART.md" ]; then
    check "QUICKSTART.md exists"
else
    warn "QUICKSTART.md missing"
fi

if [ -f "README.md" ]; then
    check "README.md exists"
else
    warn "README.md missing"
fi

# === DEPENDENCIES ===
section "Dependencies Check"

if [ -f "src/Cargo.toml" ]; then
    info "Key dependencies:"
    grep "^tokio" src/Cargo.toml | head -1 | sed 's/^/    /'
    grep "^axum" src/Cargo.toml | head -1 | sed 's/^/    /'
    grep "^serde" src/Cargo.toml | head -1 | sed 's/^/    /'
    grep "^dashmap" src/Cargo.toml | head -1 | sed 's/^/    /'
fi

# === OBSOLETE FILES CHECK ===
section "Obsolete Files (should not exist)"

OBSOLETE=0

if [ -f "docker-compose.yml" ]; then
    warn "docker-compose.yml still exists (old architecture)"
    OBSOLETE=$((OBSOLETE + 1))
fi

if [ -d "config" ]; then
    warn "config/ directory still exists (old architecture)"
    OBSOLETE=$((OBSOLETE + 1))
fi

if [ -d "docker" ]; then
    warn "docker/ directory still exists (old architecture)"
    OBSOLETE=$((OBSOLETE + 1))
fi

if [ "$OBSOLETE" -eq 0 ]; then
    check "No obsolete files from old architecture"
fi

# === SUMMARY ===
echo ""
echo "========================================"
if [ $ISSUES -eq 0 ] && [ $WARNINGS -eq 0 ]; then
    echo -e "${GREEN}✓ All checks passed!${NC}"
    echo ""
    echo "Next steps:"
    echo "  1. cd src && cargo run"
    echo "  2. Open browser to http://localhost:8080"
    echo "  3. Connect with username and start playing!"
elif [ $ISSUES -eq 0 ]; then
    echo -e "${YELLOW}⚠ Found $WARNINGS warning(s) but no critical issues${NC}"
    echo ""
    echo "You can still run the server:"
    echo "  cd src && cargo run"
else
    echo -e "${RED}✗ Found $ISSUES issue(s) and $WARNINGS warning(s)${NC}"
    echo ""
    echo "Fix the issues above, then run this script again."
fi
echo "========================================"

# === QUICK REFERENCE ===
echo ""
echo -e "${BLUE}Quick Reference:${NC}"
echo "  Build server:    cd src && cargo build --release"
echo "  Run server:      cd src && cargo run"
echo "  Debug mode:      cd src && RUST_LOG=debug cargo run"
echo "  Check compile:   cd src && cargo check"
echo "  Run tests:       cd src && cargo test"
echo ""
echo "  Add item:        Edit src/assets/definitions/items.json"
echo "  Add NPC:         Edit src/assets/definitions/npcs.json"
echo "  Spawn NPC:       Edit src/assets/spawns/npcs/*.json"
echo ""
echo "  Share (Tailscale): tailscale ip -4"
echo "  Share (ngrok):     ngrok http 8080"
echo ""
echo "  Documentation:   docs/PROJECT_CONTEXT.md"
echo "  Quick start:     docs/QUICKSTART.md"
echo ""

exit $ISSUES
