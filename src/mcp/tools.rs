use std::sync::atomic::Ordering;
use poem_mcpserver::{content::{Json, Text}, Tools};
use serde_json::json;

use crate::shared::state::{LAST_COMMAND, SSH_CONNECTIONS, SSH_CLIENTS, get_uptime};
use crate::mcp::commands::broadcast_to_clients;

// MCP Tools implementation
pub struct McpTools {}

#[Tools]
impl McpTools {
    /// Get server status information
    pub async fn get_status(&self) -> Json<serde_json::Value> {
        let status = json!({
            "status": "online",
            "ssh_connections": SSH_CONNECTIONS.load(Ordering::SeqCst),
            "uptime": get_uptime(),
            "server_time": chrono::Utc::now().to_rfc3339()
        });
        Json(status)
    }

    /// Get the last command executed through SSH
    pub async fn get_last_command(&self) -> Text<String> {
        let cmd = LAST_COMMAND.lock().await.clone();
        Text(cmd)
    }
    
    /// List available tools
    pub async fn get_tools(&self) -> Json<serde_json::Value> {
        let tools = json!({
            "tools": [
                {
                    "name": "get_status",
                    "description": "Server status information"
                },
                {
                    "name": "get_tools",
                    "description": "List available AI tools"
                },
                {
                    "name": "get_last_command",
                    "description": "Last SSH command executed"
                },
                {
                    "name": "broadcast_message",
                    "description": "Send a message to SSH clients"
                },
                {
                    "name": "get_clients",
                    "description": "Get information about connected SSH clients"
                }
            ]
        });
        Json(tools)
    }
    
    /// Broadcast a message to all SSH clients
    pub async fn broadcast_message(&self, message: String) -> Json<serde_json::Value> {
        broadcast_to_clients(&message);
        
        let result = json!({
            "success": true,
            "message": message,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });
        Json(result)
    }
    
    /// Get connected clients information
    pub async fn get_clients(&self) -> Json<serde_json::Value> {
        let mut clients_data = Vec::new();
        
        if let Ok(clients) = SSH_CLIENTS.lock() {
            for client in clients.iter() {
                clients_data.push(json!({
                    "name": client.name,
                    "uuid": client.uuid.to_string(),
                    "connected_at": client.connected_at.to_rfc3339(),
                    "last_activity": client.last_activity.to_rfc3339()
                }));
            }
        }
        
        let result = json!({
            "count": clients_data.len(),
            "clients": clients_data
        });
        Json(result)
    }
}