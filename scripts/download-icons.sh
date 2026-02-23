#!/bin/bash

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ICONS_DIR="$PROJECT_ROOT/icons"
TEMP_DIR="$(mktemp -d)"

echo -e "${GREEN}🚀 Starting Lucide Icons download...${NC}"


cleanup() {
    echo -e "${YELLOW}🧹 Cleaning up temporary files...${NC}"
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

echo -e "${BLUE}Step 1: Preparing icons directory${NC}"
mkdir -p "$ICONS_DIR"
rm -f "$ICONS_DIR"/*.svg 2>/dev/null || true

echo -e "${BLUE}Step 2: Cloning Lucide repository (sparse checkout)${NC}"
echo -e "${GREEN}📥 This may take a moment...${NC}"

cd "$TEMP_DIR"
git clone --depth 1 --filter=blob:none --sparse https://github.com/lucide-icons/lucide.git 2>&1 | \
    while IFS= read -r line; do
        echo -e "${YELLOW}   $line${NC}"
    done

cd lucide
git sparse-checkout set icons

if [ ! -d "icons" ]; then
    echo -e "${RED}❌ Error: Icons directory not found in repository${NC}"
    exit 1
fi

TOTAL_ICONS=$(find icons -name "*.svg" | wc -l | tr -d ' ')
echo -e "${GREEN}📊 Found $TOTAL_ICONS icons${NC}"

echo -e "${BLUE}Step 3: Copying icons to project${NC}"

CURRENT=0
for svg_file in icons/*.svg; do
    CURRENT=$((CURRENT + 1))
    PERCENT=$((CURRENT * 100 / TOTAL_ICONS))
    
    FILLED=$((PERCENT / 2))
    EMPTY=$((50 - FILLED))
    BAR=$(printf "%${FILLED}s" | tr ' ' '█')
    SPACE=$(printf "%${EMPTY}s" | tr ' ' '░')
    
    printf "\r${GREEN}   [${BAR}${SPACE}] %3d%% (%d/%d)${NC}" "$PERCENT" "$CURRENT" "$TOTAL_ICONS"
    
    cp "$svg_file" "$ICONS_DIR/"
done

echo ""

DOWNLOADED=$(find "$ICONS_DIR" -name "*.svg" | wc -l | tr -d ' ')

echo ""
echo -e "${GREEN}✅ Successfully downloaded $DOWNLOADED icons to $ICONS_DIR/${NC}"
echo -e "${GREEN}🎉 Done!${NC}"
