use std::sync::Arc;

use poem::{
    listener::TcpListener, 
    middleware::Tracing, 
    EndpointExt, 
    Route, 
    Server
};
use poem_mcpserver::{
    McpServer, 
    protocol::{MinecraftIdentity, MinecraftIdentityInfo},
    services::McpService,
};
use async_trait::async_trait;

// Simple MCP service implementation
struct SimpleMcpService;

#[async_trait]
impl McpService for SimpleMcpService {
    async fn authenticate(&self, identity: MinecraftIdentity) -> MinecraftIdentityInfo {
        println!("Player trying to authenticate: {}", identity.username());
        
        // Create identity info for the player
        MinecraftIdentityInfo {
            uuid: identity.uuid().clone(),
            username: identity.username().clone(),
            properties: identity.properties().clone(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt::init();
    
    // Create MCP service
    let mcp_service = SimpleMcpService;
    
    // Create MCP server
    let mcp_server = McpServer::new(mcp_service);
    
    // Create a Poem route
    let app = Route::new()
        .nest("/", poem_mcpserver::routes())
        .with(Tracing)
        .data(mcp_server);
    
    // Start the server
    println!("Starting MCP server on 0.0.0.0:25565");
    
    Server::new(TcpListener::bind("0.0.0.0:25565"))
        .run(app)
        .await?;
    
    Ok(())
}