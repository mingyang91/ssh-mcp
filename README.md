# SSH-MCP Server

![Crates.io Version](https://img.shields.io/crates/v/ssh-mcp)
![Docker Image Version](https://img.shields.io/docker/v/mingyang91/ssh-mcp)

A Rust implementation of an SSH client server with Model Context Protocol (MCP) integration, allowing Large Language Models (LLMs) to connect to a SSH server and utilize SSH features.

**Lightweight and Efficient:**
- 🚀 `ssh-mcp`: only 4.3MB binary size, 1.8MB memory footprint
- 🔄 `ssh-mcp-stdio`: only 1.5MB binary size, 2.3MB memory usage
- 🐳 `docker`: only 27.5MB final image size

## Features

- **SSH Client Integration**: Connect to SSH servers via MCP commands
- **SSH Command Execution**: Run commands on remote SSH servers
- **Port Forwarding**: Setup SSH tunnels and port forwards (enabled by default via the `port_forward` feature)
- **Session Management**: Track and manage multiple SSH sessions
- **MCP Protocol Support**: Built with poem-mcpserver to enable AI/LLM compatibility
- **Stateful Connections**: Maintain SSH sessions across multiple commands

## Installation and Integration

You have two options for installation:

### Option 1: Using Docker (Recommended)

No Rust toolchain required - just Docker!

```bash
# Pull the pre-built image from Docker Hub
docker pull mingyang91/ssh-mcp

# Run the server
docker run -p 8000:8000 mingyang91/ssh-mcp
```

Or build the image yourself:

```bash
# Build the Docker image
docker build -t ssh-mcp .

# Run the server
docker run -p 8000:8000 ssh-mcp
```

### Option 2: Installing via Cargo

Prerequisites:
- Rust 1.70.0 or later
- Cargo package manager

```bash
cargo install ssh-mcp
```

## Integration with mcpServers

To use SSH-MCP with mcpServers, add the following configuration to your mcpServers JSON configuration:

### If installed via Cargo:

```json
{
  "mcpServers": {
    "ssh": {
      "command": "ssh-mcp-stdio", 
      "args": []
    }
  }
}
```

### If using Docker:

```json
{
  "mcpServers": {
    "ssh": {
      "command": "docker",
      "args": ["run", "--entrypoint", "ssh-mcp-stdio", "-i", "--rm", "ssh-mcp"]
    }
  }
}
```

This will register the SSH handler, allowing LLMs to manage SSH connections through your MCP server.

## Usage

### Connecting to an SSH Server

#### Using Password Authentication
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

#### Using Key Authentication
```json
{
  "command": "ssh_connect",
  "params": {
    "address": "example.com:22",
    "username": "user",
    "key_path": "/path/to/private_key"
  }
}
```

#### Using SSH Agent Authentication
```json
{
  "command": "ssh_connect",
  "params": {
    "address": "example.com:22",
    "username": "user"
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

If connection fails:
```json
{
  "error": "Failed to connect: Connection refused"
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

Response:
```json
{
  "stdout": "total 32\ndrwxr-xr-x  5 user group 4096 Jan 1 12:00 .\ndrwxr-xr-x 25 user group 4096 Jan 1 12:00 ..\n-rw-r--r--  1 user group  142 Jan 1 12:00 file.txt\n",
  "stderr": "",
  "exit_code": 0
}
```

### Setting Up Port Forwarding

Note: Port forwarding is enabled by default via the `port_forward` feature flag.

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

Response:
```json
{
  "local_address": "127.0.0.1:8080",
  "remote_address": "internal-server:80",
  "active": true
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

Response:
```json
"Session c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b disconnected successfully"
```

### Listing All Active Sessions

```json
{
  "command": "ssh_list_sessions"
}
```

Response:
```json
[
  "c8a3b2e1-4f5d-6e7c-8a9b-0c1d2e3f4a5b",
  "d9b4c3a2-5e6f-7g8h-9i0j-1k2l3m4n5o6p"
]
```

## Configuration

The server can be configured through the following environment variables:

| Variable | Description             | Default |
| -------- | ----------------------- | ------- |
| MCP_PORT | Port for the MCP server | 8000    |
| RUST_LOG | Logging level           | info    |

## Features Configuration

The project uses Cargo features to enable/disable certain functionality:

| Feature      | Description                         | Default |
| ------------ | ----------------------------------- | ------- |
| port_forward | Enables SSH port forwarding support | Enabled |

To build without port forwarding:
```
cargo build --release --no-default-features
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### CI/CD Pipeline

This project uses GitHub Actions for continuous integration and deployment:

- All PRs and commits to main are automatically tested
- Tagged releases (starting with 'v') are automatically published to crates.io
- Docker images are automatically built and pushed to Docker Hub
  - Latest tag for main branch
  - Version tags for semantic versioned releases (v1.0.0, v1.0, etc.)
- To create a new release, ensure version in Cargo.toml matches the tag (e.g., v0.1.0)
- You'll need to set up the `CARGO_REGISTRY_TOKEN`, `DOCKERHUB_USERNAME`, and `DOCKERHUB_TOKEN` secrets in your GitHub repository settings

## License

This project is licensed under the MIT License - see the LICENSE file for details.