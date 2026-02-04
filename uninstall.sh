#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🗑️  Uninstalling cc-statusline...${NC}\n"

# Check if jq is available
if ! command -v jq &> /dev/null; then
    echo -e "${RED}❌ Error: jq is required but not installed.${NC}"
    exit 1
fi

SETTINGS_FILE="$HOME/.claude/settings.json"
INSTALL_BINARY="$HOME/.claude/bin/cc-statusline"

if [ ! -f "$SETTINGS_FILE" ]; then
    echo -e "${YELLOW}⚠️  No settings file found at: $SETTINGS_FILE${NC}"
    echo -e "${GREEN}✓ Nothing to uninstall${NC}"
    exit 0
fi

# Check if statusLine exists in config
if ! jq -e '.statusLine' "$SETTINGS_FILE" > /dev/null 2>&1; then
    echo -e "${YELLOW}⚠️  No statusLine configuration found${NC}"
    echo -e "${GREEN}✓ Nothing to uninstall${NC}"
    exit 0
fi

# Backup existing file
BACKUP_FILE="${SETTINGS_FILE}.backup.$(date +%s)"
cp "$SETTINGS_FILE" "$BACKUP_FILE"
echo -e "${BLUE}📋 Backup created: $BACKUP_FILE${NC}"

# Remove statusLine key
jq 'del(.statusLine)' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp" && mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
echo -e "${GREEN}✓ Removed statusLine from settings${NC}"

# Remove binary
if [ -f "$INSTALL_BINARY" ]; then
    rm "$INSTALL_BINARY"
    echo -e "${GREEN}✓ Removed binary from $INSTALL_BINARY${NC}"
else
    echo -e "${YELLOW}⚠️  Binary not found at $INSTALL_BINARY${NC}"
fi

echo -e "\n${BLUE}🎉 Uninstall complete!${NC}"
echo -e "\n${BLUE}💡 To restore settings:${NC}"
echo -e "   ${YELLOW}cp $BACKUP_FILE $SETTINGS_FILE${NC}"
echo ""
