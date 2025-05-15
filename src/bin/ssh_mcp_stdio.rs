#![deny(warnings)]
#![deny(clippy::unwrap_used)]

use dotenv::dotenv;
use poem_mcpserver::McpServer;
use ssh_mcp::mcp::McpSSHCommands;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let mcp_port: u16 = std::env::var("MCP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000);
    let mcp_addr = format!("0.0.0.0:{}", mcp_port);
    info!("Starting MCP server on {}", mcp_addr);

    poem_mcpserver::stdio::stdio(McpServer::new().tools(McpSSHCommands {})).await?;

    Ok(())
}
