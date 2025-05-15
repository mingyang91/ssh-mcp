use log::info;
use poem::{
    listener::TcpListener, 
    middleware::Tracing, 
    EndpointExt, 
    Route, 
    Server,
    Error as PoemError,
};
use poem_mcpserver::{
    streamable_http, 
    McpServer,
};

use crate::mcp::tools::McpTools;

/// Create and run the MCP server
pub async fn run_mcp_server(mcp_addr: &str) -> Result<(), PoemError> {
    info!("Starting MCP server on {}", mcp_addr);

    // Setup the poem-mcpserver endpoint
    let app = Route::new()
        .at(
            "/",
            streamable_http::endpoint(|_| {
                let mcp_server = McpServer::new()
                    .tools(McpTools {});
                
                mcp_server
            }),
        )
        .with(Tracing);

    // Run the MCP server
    let result = Server::new(TcpListener::bind(mcp_addr))
        .name("MCP Server")
        .run(app)
        .await;
        
    // Convert std::io::Error to poem::Error if needed
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(PoemError::from_string(e.to_string()))
    }
}

/// Create routes for MCP API
pub fn create_mcp_routes() -> Route {
    Route::new()
        .at(
            "/",
            streamable_http::endpoint(|_| {
                McpServer::new()
                    .tools(McpTools {})
            }),
        )
}