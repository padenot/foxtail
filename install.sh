#!/bin/bash
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Installing foxtail...${NC}\n"

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo -e "${RED}❌ Error: jq is required but not installed.${NC}"
    echo -e "Install it with:"
    echo -e "  - Ubuntu/Debian: ${YELLOW}sudo apt install jq${NC}"
    echo -e "  - Fedora/RHEL: ${YELLOW}sudo dnf install jq${NC}"
    echo -e "  - macOS: ${YELLOW}brew install jq${NC}"
    echo -e "  - Arch: ${YELLOW}sudo pacman -S jq${NC}"
    exit 1
fi

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_BINARY="${SCRIPT_DIR}/target/release/foxtail"
INSTALL_DIR="$HOME/.claude/bin"
INSTALL_BINARY="$INSTALL_DIR/foxtail"

# Step 1: Build the binary
echo -e "${BLUE}📦 Building foxtail...${NC}"
cd "$SCRIPT_DIR"
if cargo build --release; then
    echo -e "${GREEN}✓ Build successful${NC}\n"
else
    echo -e "${RED}❌ Build failed${NC}"
    exit 1
fi

# Verify binary exists
if [ ! -f "$BUILD_BINARY" ]; then
    echo -e "${RED}❌ Binary not found at: $BUILD_BINARY${NC}"
    exit 1
fi

# Step 2: Install binary to ~/.claude/bin
echo -e "${BLUE}📦 Installing binary to $INSTALL_DIR...${NC}"
mkdir -p "$INSTALL_DIR"
cp "$BUILD_BINARY" "$INSTALL_BINARY"
chmod +x "$INSTALL_BINARY"
echo -e "${GREEN}✓ Binary installed${NC}\n"

# Step 3: Update Claude Code settings
SETTINGS_DIR="$HOME/.claude"
SETTINGS_FILE="$SETTINGS_DIR/settings.json"

# Create .claude directory if it doesn't exist
if [ ! -d "$SETTINGS_DIR" ]; then
    echo -e "${BLUE}📁 Creating $SETTINGS_DIR directory...${NC}"
    mkdir -p "$SETTINGS_DIR"
fi

# Prepare the statusLine configuration
STATUSLINE_CONFIG=$(jq -n \
    --arg cmd "$INSTALL_BINARY" \
    '{
        statusLine: {
            type: "command",
            command: $cmd,
            padding: 0
        }
    }')

# Update or create settings.json
if [ -f "$SETTINGS_FILE" ]; then
    echo -e "${BLUE}📝 Updating existing settings.json...${NC}"

    # Backup existing file
    BACKUP_FILE="${SETTINGS_FILE}.backup.$(date +%s)"
    cp "$SETTINGS_FILE" "$BACKUP_FILE"
    echo -e "${YELLOW}   Backup created: $BACKUP_FILE${NC}"

    # Merge configurations (new statusLine config overwrites existing one)
    MERGED_CONFIG=$(jq -s '.[0] * .[1]' "$SETTINGS_FILE" <(echo "$STATUSLINE_CONFIG"))

    # Write merged config back
    echo "$MERGED_CONFIG" > "$SETTINGS_FILE"
    echo -e "${GREEN}✓ Settings updated${NC}"
else
    echo -e "${BLUE}📝 Creating new settings.json...${NC}"
    echo "$STATUSLINE_CONFIG" > "$SETTINGS_FILE"
    echo -e "${GREEN}✓ Settings created${NC}"
fi

# Step 4: Test the statusline
echo -e "\n${BLUE}🧪 Testing statusline...${NC}"
TEST_JSON='{
  "hook_event_name": "Status",
  "session_id": "install-test-'$(date +%s)'",
  "transcript_path": "/tmp/transcript.json",
  "cwd": "'$SCRIPT_DIR'",
  "model": {"id": "test", "display_name": "Test"},
  "workspace": {"current_dir": "'$SCRIPT_DIR'", "project_dir": "'$SCRIPT_DIR'"},
  "version": "1.0.0",
  "output_style": {"name": "default"},
  "cost": {"total_cost_usd": 0.001, "total_duration_ms": 1000, "total_api_duration_ms": 100, "total_lines_added": 10, "total_lines_removed": 2},
  "context_window": {"total_input_tokens": 1000, "total_output_tokens": 500, "context_window_size": 200000, "used_percentage": 0.75, "remaining_percentage": 99.25, "current_usage": {"input_tokens": 800, "output_tokens": 200, "cache_creation_input_tokens": 100, "cache_read_input_tokens": 50}}
}'

if echo "$TEST_JSON" | "$INSTALL_BINARY"; then
    echo -e "\n${GREEN}✓ Test successful${NC}"
else
    echo -e "\n${RED}❌ Test failed${NC}"
    exit 1
fi

# Summary
echo -e "\n${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}✓ Installation complete!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "\n${BLUE}📍 Binary location:${NC}"
echo -e "   $INSTALL_BINARY"
echo -e "\n${BLUE}⚙️  Config file:${NC}"
echo -e "   $SETTINGS_FILE"
echo -e "\n${BLUE}🎉 Next steps:${NC}"
echo -e "   1. Restart Claude Code (if running)"
echo -e "   2. The statusline should appear at the bottom"
echo -e "   3. View examples: ${YELLOW}cat $SCRIPT_DIR/EXAMPLES.md${NC}"
echo -e "\n${BLUE}💡 To uninstall:${NC}"
echo -e "   Run: ${YELLOW}$SCRIPT_DIR/uninstall.sh${NC}"
echo -e "   Or manually remove: ${YELLOW}rm $INSTALL_BINARY${NC}"
echo ""
