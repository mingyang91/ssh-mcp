# SSH-MCP Integration for Windsurf

This document provides instructions for integrating the SSH-MCP server with Windsurf.

## Overview

SSH-MCP provides SSH capabilities to Cascade through the Model Context Protocol (MCP). This integration allows you to:

- Connect to remote SSH servers
- Execute commands on remote servers
- Set up port forwarding
- Manage SSH sessions

## Installation

### Option 1: Build from Source

1. Clone this repository:
   ```bash
   git clone https://github.com/taciclei/ssh-mcp.git
   cd ssh-mcp
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. The binary will be available at `target/release/ssh-mcp-stdio`

### Option 2: Install via Cargo

```bash
cargo install ssh-mcp
```

## Configuration

Add the following to your Windsurf MCP configuration file (usually at `~/.codeium/windsurf/mcp_config.json`):

```json
{
  "mcpServers": {
    "ssh-mcp": {
      "command": "/path/to/ssh-mcp/target/release/ssh-mcp-stdio",
      "args": [],
      "env": {
        "SSH_KEY_PATH": "/absolute/path/to/.ssh/id_ed25519",
        "SSH_DEFAULT_HOST": "username@hostname"
      }
    }
  }
}
```

**Important notes:**
- Make sure to use absolute paths for the SSH key
- `SSH_DEFAULT_HOST` should be in the format `username@hostname`

## Usage in Cascade

Once configured, you can use the following commands in Cascade:

### Connect to an SSH server

```
ssh_connect(address="username@hostname:port", key_path="/path/to/key")
```

### Execute commands

```
ssh_execute(session_id="session_id_from_connect", command="your command")
```

### List active sessions

```
ssh_list_sessions()
```

### Disconnect a session

```
ssh_disconnect(session_id="session_id_from_connect")
```

## Troubleshooting

- If you encounter connection issues, make sure your SSH key path is an absolute path
- Check that your target server accepts the authentication method you're using
- Verify the server is reachable from your current network

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
