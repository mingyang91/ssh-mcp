#!/bin/bash
# SSH-MCP Windsurf Setup Script
# This script helps to install and configure SSH-MCP for Windsurf

set -e  # Exit on error

# Colors for prettier output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== SSH-MCP Windsurf Setup ===${NC}"
echo ""

# Check if Cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed. Please install Rust and Cargo first.${NC}"
    echo "Visit https://rustup.rs/ for installation instructions."
    exit 1
fi

# Get the user's home directory
HOME_DIR="$HOME"
WINDSURF_CONFIG_DIR="$HOME_DIR/.codeium/windsurf"
MCP_CONFIG_FILE="$WINDSURF_CONFIG_DIR/mcp_config.json"

# Build SSH-MCP
echo -e "${YELLOW}Building SSH-MCP...${NC}"
cargo build --release
echo -e "${GREEN}Build complete!${NC}"

# Get absolute path to the built binary
BINARY_PATH="$(pwd)/target/release/ssh-mcp-stdio"

# Get SSH key path
echo ""
echo -e "${YELLOW}SSH Configuration${NC}"
read -p "Enter the path to your SSH private key [~/.ssh/id_ed25519]: " SSH_KEY_PATH
SSH_KEY_PATH=${SSH_KEY_PATH:-~/.ssh/id_ed25519}

# Expand tilde to home directory
if [[ $SSH_KEY_PATH == ~/* ]]; then
    SSH_KEY_PATH="${HOME_DIR}/${SSH_KEY_PATH:2}"
fi

# Get default SSH host
read -p "Enter your default SSH host (format: username@hostname): " DEFAULT_SSH_HOST

# Check if the Windsurf MCP config directory exists
if [ ! -d "$WINDSURF_CONFIG_DIR" ]; then
    echo -e "${YELLOW}Creating Windsurf MCP config directory...${NC}"
    mkdir -p "$WINDSURF_CONFIG_DIR"
fi

# Check if the MCP config file exists
if [ ! -f "$MCP_CONFIG_FILE" ]; then
    echo -e "${YELLOW}Creating MCP config file...${NC}"
    # Create a basic MCP config file
    cat > "$MCP_CONFIG_FILE" << EOF
{
  "mcpServers": {
    "ssh-mcp": {
      "command": "$BINARY_PATH",
      "args": [],
      "env": {
        "SSH_KEY_PATH": "$SSH_KEY_PATH",
        "SSH_DEFAULT_HOST": "$DEFAULT_SSH_HOST"
      }
    }
  }
}
EOF
else
    # Check if the file is valid JSON
    if ! jq empty "$MCP_CONFIG_FILE" 2>/dev/null; then
        echo -e "${RED}Error: MCP config file is not valid JSON. Please fix it manually.${NC}"
        exit 1
    fi

    # Check if the mcpServers key exists
    if ! jq -e '.mcpServers' "$MCP_CONFIG_FILE" >/dev/null 2>&1; then
        echo -e "${YELLOW}Adding mcpServers to MCP config file...${NC}"
        jq '. + {"mcpServers":{}}' "$MCP_CONFIG_FILE" > "$MCP_CONFIG_FILE.tmp"
        mv "$MCP_CONFIG_FILE.tmp" "$MCP_CONFIG_FILE"
    fi

    # Add or update the ssh-mcp server
    echo -e "${YELLOW}Updating MCP config file...${NC}"
    jq --arg path "$BINARY_PATH" \
       --arg key_path "$SSH_KEY_PATH" \
       --arg host "$DEFAULT_SSH_HOST" \
       '.mcpServers["ssh-mcp"] = {"command": $path, "args": [], "env": {"SSH_KEY_PATH": $key_path, "SSH_DEFAULT_HOST": $host}}' \
       "$MCP_CONFIG_FILE" > "$MCP_CONFIG_FILE.tmp"
    mv "$MCP_CONFIG_FILE.tmp" "$MCP_CONFIG_FILE"
fi

echo ""
echo -e "${GREEN}SSH-MCP has been successfully configured for Windsurf!${NC}"
echo -e "${YELLOW}Please restart Windsurf to apply the changes.${NC}"
