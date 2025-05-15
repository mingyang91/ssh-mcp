# SSH-MCP Server

A Rust implementation of an SSH client server with Model Context Protocol (MCP) integration, allowing Large Language Models (LLMs) to connect to a SSH server and utilize SSH features.

## Features

- **SSH Client Integration**: Connect to SSH servers via MCP commands
- **SSH Command Execution**: Run commands on remote SSH servers
- **Port Forwarding**: Setup SSH tunnels and port forwards
- **Session Management**: Track and manage multiple SSH sessions
- **MCP Protocol Support**: Built with poem-mcpserver to enable AI/LLM compatibility
- **Stateful Connections**: Maintain SSH sessions across multiple commands

## Prerequisites

- Rust 1.70.0 or later
- Cargo package manager

## Getting Started

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/ssh-mcp.git
   cd ssh-mcp
   ```

2. Create a `.env` file (use `.env.example` as a template):
   ```
   cp .env.example .env
   ```

3. Configure the environment variables in the `.env` file.

4. Build the project:
   ```
   cargo build --release
   ```

5. Run the server:
   ```
   cargo run --release
   ```

## Usage

### Connecting to an SSH Server

```json
{
  "command": "ssh_connect",
  "params": {
    "address": "example.com:22",
    "username": "user",
    "password": "password"
  }
}
```

Response:
```json
{
  "session_id": "c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b",
  "message": "Successfully connected to user@example.com:22",
  "authenticated": true
}
```

### Executing Commands

```json
{
  "command": "ssh_execute",
  "params": {
    "session_id": "c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b",
    "command": "ls -la"
  }
}
```

### Setting Up Port Forwarding

```json
{
  "command": "ssh_forward",
  "params": {
    "session_id": "c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b",
    "local_port": 8080,
    "remote_address": "internal-server",
    "remote_port": 80
  }
}
```

### Disconnecting a Session

```json
{
  "command": "ssh_disconnect",
  "params": {
    "session_id": "c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b"
  }
}
```

### Listing All Active Sessions

```json
{
  "command": "ssh_list_sessions"
}
```

## Configuration

The server can be configured through the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| MCP_PORT | Port for the MCP server | 8000 |
| HOST | Host address to bind to | 0.0.0.0 |
| RUST_LOG | Logging level | info |

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.