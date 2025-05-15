use log::info;

/// Process MCP command input from SSH
pub fn process_mcp_command(command: &str) -> String {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        return "Unknown MCP command. Try 'mcp help' for available commands.\r\n$ ".to_string();
    }

    match parts[1] {
        "info" => {
            format!("SSH MCP Server Information\r\n\
                    Version: {}\r\n\
                    SSH Port: {}\r\n\
                    MCP Port: {}\r\n\
                    $ ", 
                    env!("CARGO_PKG_VERSION"),
                    std::env::var("SSH_PORT").unwrap_or_else(|_| "22".to_string()),
                    std::env::var("MCP_PORT").unwrap_or_else(|_| "8000".to_string()))
        },
        "status" => {
            let connections = crate::shared::state::SSH_CONNECTIONS.load(std::sync::atomic::Ordering::SeqCst);
            let uptime = crate::shared::state::get_uptime();
            
            format!("Server Status: Online\r\n\
                    Active SSH Connections: {}\r\n\
                    Uptime: {}\r\n\
                    Server Time: {}\r\n\
                    $ ",
                    connections,
                    uptime,
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"))
        },
        "tools" => {
            "Available MCP Tools:\r\n\
             - get_status: Server status information\r\n\
             - get_tools: List available AI tools\r\n\
             - get_last_command: Last SSH command executed\r\n\
             - broadcast_message: Send a message to SSH clients\r\n\
             - get_clients: Get connected clients information\r\n\
             $ ".to_string()
        },
        "help" => {
            "SSH MCP Commands:\r\n\
             - mcp info: Show server information\r\n\
             - mcp status: Show server status\r\n\
             - mcp tools: Show available MCP tools\r\n\
             - mcp help: Show this help message\r\n\
             - mcp clients: List connected clients\r\n\
             - mcp broadcast <message>: Send message to all connected clients\r\n\
             $ ".to_string()
        },
        "clients" => {
            let mut response = String::from("Connected Clients:\r\n");
            
            if let Ok(clients) = crate::shared::state::SSH_CLIENTS.lock() {
                if clients.is_empty() {
                    response.push_str("No clients connected\r\n");
                } else {
                    for (i, client) in clients.iter().enumerate() {
                        let duration = chrono::Utc::now().signed_duration_since(client.connected_at);
                        let hours = duration.num_hours();
                        let mins = duration.num_minutes() % 60;
                        let secs = duration.num_seconds() % 60;
                        
                        response.push_str(&format!("{}. {} (UUID: {})\r\n   Connected: {} ({:02}:{:02}:{:02} ago)\r\n",
                            i + 1,
                            client.name,
                            client.uuid,
                            client.connected_at.format("%Y-%m-%d %H:%M:%S UTC"),
                            hours, mins, secs));
                    }
                }
            } else {
                response.push_str("Error accessing client list\r\n");
            }
            
            response.push_str("$ ");
            response
        },
        "broadcast" => {
            if parts.len() < 3 {
                "Usage: mcp broadcast <message>\r\n$ ".to_string()
            } else {
                let message = parts[2..].join(" ");
                broadcast_to_clients(&message);
                format!("Message broadcast: {}\r\n$ ", message)
            }
        },
        _ => format!("Unknown MCP command: {}. Try 'mcp help' for available commands.\r\n$ ", parts[1])
    }
}

/// Function to broadcast messages to all SSH clients
pub fn broadcast_to_clients(message: &str) {
    info!("Broadcasting message to all clients: {}", message);
    
    // For now just log the message
    // In a more complete implementation, we would:
    // 1. Maintain a collection of active SSH channels
    // 2. Format message with appropriate terminal codes
    // 3. Iterate through all channels and send the message
    
    // Note: Implementation is challenging because:
    // - russh channels are owned by their respective handlers
    // - Broadcasting requires access to these channels from another context
    // - This would need a shared state with proper synchronization
}