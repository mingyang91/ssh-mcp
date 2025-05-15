use poem::endpoint::StaticFilesEndpoint;
use poem::get;
use poem::handler;
use poem::web::Html;
use poem::{Route, Server};
use poem::listener::TcpListener;
use reqwest::Client;
use serde_json::{json, Value};
use std::error::Error;

#[handler]
async fn index() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>SSH-MCP Client Example</title>
            <style>
                body { font-family: Arial, sans-serif; margin: 20px; }
                .container { max-width: 800px; margin: 0 auto; }
                textarea { width: 100%; height: 200px; margin-top: 10px; }
                input, button { margin-top: 10px; padding: 8px; }
                .output { background-color: #f5f5f5; padding: 10px; margin-top: 10px; border-radius: 5px; }
                .form-group { margin-bottom: 15px; }
                label { display: block; margin-bottom: 5px; }
            </style>
        </head>
        <body>
            <div class="container">
                <h1>SSH-MCP Client Example</h1>
                
                <div class="form-group">
                    <label for="host">SSH Server:</label>
                    <input type="text" id="host" placeholder="example.com" />
                    <input type="text" id="port" placeholder="22" style="width: 60px;" />
                </div>
                
                <div class="form-group">
                    <label for="username">Username:</label>
                    <input type="text" id="username" placeholder="username" />
                </div>
                
                <div class="form-group">
                    <label for="password">Password:</label>
                    <input type="password" id="password" placeholder="password" />
                </div>
                
                <button id="connect">Connect</button>
                <button id="disconnect" disabled>Disconnect</button>
                
                <hr />
                
                <div class="form-group">
                    <label for="command">SSH Command:</label>
                    <input type="text" id="command" placeholder="ls -la" style="width: 80%;" disabled />
                    <button id="execute" disabled>Execute</button>
                </div>
                
                <div class="form-group">
                    <label for="localPort">Port Forwarding:</label>
                    <input type="text" id="localPort" placeholder="Local Port" style="width: 100px;" disabled />
                    <input type="text" id="remoteHost" placeholder="Remote Host" style="width: 150px;" disabled />
                    <input type="text" id="remotePort" placeholder="Remote Port" style="width: 100px;" disabled />
                    <button id="forward" disabled>Forward</button>
                </div>
                
                <div class="form-group">
                    <label for="output">Output:</label>
                    <textarea id="output" readonly></textarea>
                </div>
            </div>
            
            <script>
                let sessionId = null;
                
                document.getElementById('connect').addEventListener('click', async () => {
                    const host = document.getElementById('host').value;
                    const port = document.getElementById('port').value || '22';
                    const username = document.getElementById('username').value;
                    const password = document.getElementById('password').value;
                    
                    const address = `${host}:${port}`;
                    
                    try {
                        const response = await fetch('/api/ssh_connect', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                address,
                                username,
                                password
                            }),
                        });
                        
                        const data = await response.json();
                        document.getElementById('output').value = JSON.stringify(data, null, 2);
                        
                        if (data.session_id) {
                            sessionId = data.session_id;
                            document.getElementById('connect').disabled = true;
                            document.getElementById('disconnect').disabled = false;
                            document.getElementById('command').disabled = false;
                            document.getElementById('execute').disabled = false;
                            document.getElementById('localPort').disabled = false;
                            document.getElementById('remoteHost').disabled = false;
                            document.getElementById('remotePort').disabled = false;
                            document.getElementById('forward').disabled = false;
                        }
                    } catch (error) {
                        document.getElementById('output').value = `Error: ${error.message}`;
                    }
                });
                
                document.getElementById('disconnect').addEventListener('click', async () => {
                    if (!sessionId) return;
                    
                    try {
                        const response = await fetch('/api/ssh_disconnect', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                session_id: sessionId
                            }),
                        });
                        
                        const data = await response.json();
                        document.getElementById('output').value = JSON.stringify(data, null, 2);
                        
                        sessionId = null;
                        document.getElementById('connect').disabled = false;
                        document.getElementById('disconnect').disabled = true;
                        document.getElementById('command').disabled = true;
                        document.getElementById('execute').disabled = true;
                        document.getElementById('localPort').disabled = true;
                        document.getElementById('remoteHost').disabled = true;
                        document.getElementById('remotePort').disabled = true;
                        document.getElementById('forward').disabled = true;
                    } catch (error) {
                        document.getElementById('output').value = `Error: ${error.message}`;
                    }
                });
                
                document.getElementById('execute').addEventListener('click', async () => {
                    if (!sessionId) return;
                    
                    const command = document.getElementById('command').value;
                    
                    try {
                        const response = await fetch('/api/ssh_execute', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                session_id: sessionId,
                                command
                            }),
                        });
                        
                        const data = await response.json();
                        document.getElementById('output').value = JSON.stringify(data, null, 2);
                    } catch (error) {
                        document.getElementById('output').value = `Error: ${error.message}`;
                    }
                });
                
                document.getElementById('forward').addEventListener('click', async () => {
                    if (!sessionId) return;
                    
                    const localPort = document.getElementById('localPort').value;
                    const remoteHost = document.getElementById('remoteHost').value;
                    const remotePort = document.getElementById('remotePort').value;
                    
                    try {
                        const response = await fetch('/api/ssh_forward', {
                            method: 'POST',
                            headers: {
                                'Content-Type': 'application/json',
                            },
                            body: JSON.stringify({
                                session_id: sessionId,
                                local_port: parseInt(localPort),
                                remote_address: remoteHost,
                                remote_port: parseInt(remotePort)
                            }),
                        });
                        
                        const data = await response.json();
                        document.getElementById('output').value = JSON.stringify(data, null, 2);
                    } catch (error) {
                        document.getElementById('output').value = `Error: ${error.message}`;
                    }
                });
            </script>
        </body>
        </html>
        "#
    )
}

async fn proxy_request(url: &str, body: Value) -> Result<Value, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .post(url)
        .json(&body)
        .send()
        .await?
        .json::<Value>()
        .await?;
    
    Ok(response)
}

#[handler]
async fn ssh_connect(body: poem::web::Json<Value>) -> poem::web::Json<Value> {
    let mcp_server_url = "http://127.0.0.1:8000";
    let request = json!({
        "tool_name": "ssh_connect",
        "parameters": {
            "address": body.0.get("address").and_then(|v| v.as_str()).unwrap_or(""),
            "username": body.0.get("username").and_then(|v| v.as_str()).unwrap_or(""),
            "password": body.0.get("password").and_then(|v| v.as_str())
        }
    });
    
    match proxy_request(mcp_server_url, request).await {
        Ok(response) => poem::web::Json(response),
        Err(e) => poem::web::Json(json!({"error": e.to_string()})),
    }
}

#[handler]
async fn ssh_execute(body: poem::web::Json<Value>) -> poem::web::Json<Value> {
    let mcp_server_url = "http://127.0.0.1:8000";
    let request = json!({
        "tool_name": "ssh_execute",
        "parameters": {
            "session_id": body.0.get("session_id").and_then(|v| v.as_str()).unwrap_or(""),
            "command": body.0.get("command").and_then(|v| v.as_str()).unwrap_or("")
        }
    });
    
    match proxy_request(mcp_server_url, request).await {
        Ok(response) => poem::web::Json(response),
        Err(e) => poem::web::Json(json!({"error": e.to_string()})),
    }
}

#[handler]
async fn ssh_forward(body: poem::web::Json<Value>) -> poem::web::Json<Value> {
    let mcp_server_url = "http://127.0.0.1:8000";
    let request = json!({
        "tool_name": "ssh_forward",
        "parameters": {
            "session_id": body.0.get("session_id").and_then(|v| v.as_str()).unwrap_or(""),
            "local_port": body.0.get("local_port").and_then(|v| v.as_u64()).unwrap_or(0),
            "remote_address": body.0.get("remote_address").and_then(|v| v.as_str()).unwrap_or(""),
            "remote_port": body.0.get("remote_port").and_then(|v| v.as_u64()).unwrap_or(0)
        }
    });
    
    match proxy_request(mcp_server_url, request).await {
        Ok(response) => poem::web::Json(response),
        Err(e) => poem::web::Json(json!({"error": e.to_string()})),
    }
}

#[handler]
async fn ssh_disconnect(body: poem::web::Json<Value>) -> poem::web::Json<Value> {
    let mcp_server_url = "http://127.0.0.1:8000";
    let request = json!({
        "tool_name": "ssh_disconnect",
        "parameters": {
            "session_id": body.0.get("session_id").and_then(|v| v.as_str()).unwrap_or("")
        }
    });
    
    match proxy_request(mcp_server_url, request).await {
        Ok(response) => poem::web::Json(response),
        Err(e) => poem::web::Json(json!({"error": e.to_string()})),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Route::new()
        .at("/", get(index))
        .at("/api/ssh_connect", get(ssh_connect).post(ssh_connect))
        .at("/api/ssh_execute", get(ssh_execute).post(ssh_execute))
        .at("/api/ssh_forward", get(ssh_forward).post(ssh_forward))
        .at("/api/ssh_disconnect", get(ssh_disconnect).post(ssh_disconnect));

    println!("Starting example server at http://localhost:3000");
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}