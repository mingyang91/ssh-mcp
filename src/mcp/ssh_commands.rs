use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::Path;
use std::sync::Arc;

use log::{debug, error, info};
use once_cell::sync::Lazy;
use poem_mcpserver::{content::Json, Tools};
use serde::{Deserialize, Serialize};
use ssh2::Session;
use tokio::sync::Mutex;
use uuid::Uuid;

// Global storage for active SSH sessions
static SSH_SESSIONS: Lazy<Mutex<std::collections::HashMap<String, Arc<Mutex<Session>>>>> =
    Lazy::new(|| Mutex::new(std::collections::HashMap::new()));

#[derive(Debug, Serialize, Deserialize)]
pub struct SshConnectResponse {
    session_id: String,
    message: String,
    authenticated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SshCommandResponse {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortForwardingResponse {
    local_address: String,
    remote_address: String,
    active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
}

pub struct McpSSHCommands;

#[Tools]
impl McpSSHCommands {
    /// Connect to an SSH server and store the session
    async fn ssh_connect(
        &self,
        address: String,
        username: String,
        password: Option<String>,
        key_path: Option<String>,
    ) -> Json<Result<SshConnectResponse, ErrorResponse>> {
        info!("Attempting SSH connection to {}@{}", username, address);

        match connect_to_ssh(
            &address,
            &username,
            password.as_deref(),
            key_path.as_deref(),
        )
        .await
        {
            Ok(session) => {
                // Generate a unique session ID
                let session_id = Uuid::new_v4().to_string();

                // Store the session
                let mut sessions = SSH_SESSIONS.lock().await;
                sessions.insert(session_id.clone(), Arc::new(Mutex::new(session)));

                Json(Ok(SshConnectResponse {
                    session_id,
                    message: format!("Successfully connected to {}@{}", username, address),
                    authenticated: true,
                }))
            }
            Err(e) => {
                error!("SSH connection failed: {}", e);
                Json(Err(ErrorResponse {
                    error: e.to_string(),
                }))
            }
        }
    }

    /// Execute a command on a connected SSH session
    async fn ssh_execute(
        &self,
        session_id: String,
        command: String,
    ) -> Json<Result<SshCommandResponse, ErrorResponse>> {
        info!(
            "Executing command on SSH session {}: {}",
            session_id, command
        );

        let sessions = SSH_SESSIONS.lock().await;
        let Some(session_arc) = sessions.get(&session_id) else {
            return Json(Err(ErrorResponse {
                error: format!("No active SSH session with ID: {}", session_id),
            }));
        };
        let session = session_arc.lock().await;
        let res = execute_ssh_command(&session, &command).await.map_err(|e| {
            error!("Command execution failed: {}", e);
            ErrorResponse {
                error: e.to_string(),
            }
        });
        Json(res)
    }

    /// Setup port forwarding on an existing SSH session
    async fn ssh_forward(
        &self,
        session_id: String,
        local_port: u16,
        remote_address: String,
        remote_port: u16,
    ) -> Json<Result<PortForwardingResponse, ErrorResponse>> {
        info!(
            "Setting up port forwarding from local port {} to {}:{} using session {}",
            local_port, remote_address, remote_port, session_id
        );

        let sessions = SSH_SESSIONS.lock().await;
        let Some(session_arc) = sessions.get(&session_id) else {
            return Json(Err(ErrorResponse {
                error: format!("No active SSH session with ID: {}", session_id),
            }));
        };
        let session = session_arc.lock().await;
        match setup_port_forwarding(&session, local_port, &remote_address, remote_port).await {
            Ok(local_addr) => Json(Ok(PortForwardingResponse {
                local_address: local_addr.to_string(),
                remote_address: format!("{}:{}", remote_address, remote_port),
                active: true,
            })),
            Err(e) => {
                error!("Port forwarding setup failed: {}", e);
                Json(Err(ErrorResponse {
                    error: e.to_string(),
                }))
            }
        }
    }

    /// Disconnect an SSH session
    async fn ssh_disconnect(&self, session_id: String) -> Json<Result<String, ErrorResponse>> {
        info!("Disconnecting SSH session: {}", session_id);

        let mut sessions = SSH_SESSIONS.lock().await;
        if sessions.remove(&session_id).is_some() {
            Json(Ok(format!(
                "Session {} disconnected successfully",
                session_id
            )))
        } else {
            Json(Err(ErrorResponse {
                error: format!("No active SSH session with ID: {}", session_id),
            }))
        }
    }

    /// List all active SSH sessions
    async fn ssh_list_sessions(&self) -> Json<Result<Vec<String>, ErrorResponse>> {
        let sessions = SSH_SESSIONS.lock().await;
        let session_ids: Vec<String> = sessions.keys().cloned().collect();

        Json(Ok(session_ids))
    }
}

// Implementation functions for SSH operations

async fn connect_to_ssh(
    address: &str,
    username: &str,
    password: Option<&str>,
    key_path: Option<&str>,
) -> Result<Session, String> {
    let tcp = TcpStream::connect(address).map_err(|e| format!("Failed to connect: {}", e))?;
    let mut sess = Session::new().map_err(|e| format!("Failed to create SSH session: {}", e))?;

    sess.set_tcp_stream(tcp);
    sess.handshake()
        .map_err(|e| format!("SSH handshake failed: {}", e))?;

    // Authenticate with either password or key
    if let Some(password) = password {
        sess.userauth_password(username, password)
            .map_err(|e| format!("Password authentication failed: {}", e))?;
    } else if let Some(key_path) = key_path {
        sess.userauth_pubkey_file(username, None, Path::new(key_path), None)
            .map_err(|e| format!("Key authentication failed: {}", e))?;
    } else {
        // Try agent authentication
        sess.userauth_agent(username)
            .map_err(|e| format!("Agent authentication failed: {}", e))?;
    }

    if !sess.authenticated() {
        return Err("Authentication failed".to_string());
    }

    Ok(sess)
}

async fn execute_ssh_command(sess: &Session, command: &str) -> Result<SshCommandResponse, String> {
    let mut channel = sess
        .channel_session()
        .map_err(|e| format!("Failed to open channel: {}", e))?;

    channel
        .exec(command)
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    let mut stdout = String::new();
    channel
        .read_to_string(&mut stdout)
        .map_err(|e| format!("Failed to read stdout: {}", e))?;

    let mut stderr = String::new();
    channel
        .stderr()
        .read_to_string(&mut stderr)
        .map_err(|e| format!("Failed to read stderr: {}", e))?;

    let exit_code = channel
        .exit_status()
        .map_err(|e| format!("Failed to get exit status: {}", e))?;

    channel
        .wait_close()
        .map_err(|e| format!("Failed to close channel: {}", e))?;

    Ok(SshCommandResponse {
        stdout,
        stderr,
        exit_code,
    })
}

async fn setup_port_forwarding(
    sess: &Session,
    local_port: u16,
    remote_address: &str,
    remote_port: u16,
) -> Result<SocketAddr, String> {
    // Create a TCP listener for the local port
    let listener_addr = format!("127.0.0.1:{}", local_port);
    let listener = std::net::TcpListener::bind(&listener_addr)
        .map_err(|e| format!("Failed to bind to local port {}: {}", local_port, e))?;

    let local_addr = listener
        .local_addr()
        .map_err(|e| format!("Failed to get local address: {}", e))?;

    // Create a new session instance that can be moved into the spawned task
    let sess_clone = sess.clone();

    // Clone needed values for the spawned task to avoid borrowing issues
    let remote_addr_clone = remote_address.to_string();
    let remote_port_clone = remote_port;

    // Start a separate thread to handle port forwarding connections
    std::thread::spawn(move || {
        debug!("Port forwarding active on {}", local_addr);

        for stream in listener.incoming() {
            match stream {
                Ok(local_stream) => {
                    let client_addr = match local_stream.peer_addr() {
                        Ok(addr) => addr,
                        Err(_) => continue,
                    };

                    debug!("New connection from {} to forwarded port", client_addr);

                    // For each connection, create a channel to the remote destination
                    match sess_clone.channel_direct_tcpip(
                        &remote_addr_clone,
                        remote_port_clone,
                        None,
                    ) {
                        Ok(mut remote_channel) => {
                            // Handle the forwarding in a separate thread
                            std::thread::spawn(move || {
                                let mut buffer = [0; 8192];
                                let mut local_stream = local_stream;

                                loop {
                                    match local_stream.read(&mut buffer) {
                                        Ok(0) => break, // EOF
                                        Ok(n) => {
                                            if remote_channel.write(&buffer[..n]).is_err() {
                                                break;
                                            }
                                        }
                                        Err(_) => break,
                                    }
                                }

                                // Cleanup
                                let _ = remote_channel.close();
                                debug!("Port forwarding connection closed");
                            });
                        }
                        Err(e) => {
                            error!("Failed to create direct channel: {}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error accepting connection: {}", e);
                    break;
                }
            }
        }
    });

    Ok(local_addr)
}
