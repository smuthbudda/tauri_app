use std::sync::Mutex;
use tauri::{AppHandle, Emitter};
use tokio::{net::UdpSocket, task};

use crate::app_state::{AppState, ServerDetails};

#[tauri::command]
pub fn get_servers(state: tauri::State<'_, Mutex<AppState>>) -> Result<String, String>{
    let app_state = state.lock().map_err(|_| "Failed to lock AppState".to_string())?;
    let server_list = serde_json::to_string(&app_state.servers).map_err(|e| e.to_string())?;
    Ok(server_list)
}


/// Creates a server on the given port and tracks the ports in use
#[tauri::command]
pub async fn create_server(
    state: tauri::State<'_, Mutex<AppState>>, 
    app_handle: tauri::AppHandle,
    port: String,
    name: String,
) -> Result<String, String> {
    let address = "0.0.0.0";
    
    print!("Creating server on {}:{}", address, port);
    let addr: String = format!("{}:{}", address, port);
    {
        // Update AppState
        let mut app_state = state.lock().map_err(|_| "Failed to lock AppState".to_string())?;
        let server_details = ServerDetails::new(address.to_string(), port.clone(), name.clone());
        
        if app_state.servers.contains(&server_details) {
            return Err(format!("Port {} is already in use", port));
        }
        app_state.add_server(server_details);
    }

    // Spawn the server
    let app_handle_clone = app_handle.clone();
    task::spawn(async move {
        if let Err(e) = start_server(&addr, app_handle_clone).await {
            eprintln!("Server error on {}: {}", &addr, e);
        }
    });

    // Get the app state and return server list
    let app_state = state.lock().map_err(|_| "Failed to lock AppState".to_string())?;
    let server_list = serde_json::to_string(&app_state.servers).map_err(|e| e.to_string())?;
    Ok(server_list)
}



/// Starts the server and handles emitting events to the frontend
 async fn start_server(addr: &str, app_handle: AppHandle) -> Result<(), String> {
    // Attempt to bind the UDP socket
    let sock = UdpSocket::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind socket: {}", e))?;

    let mut buf = [0; 1024];
    loop {
        // Receive data from the socket
        match sock.recv_from(&mut buf).await {
            Ok((len, client_addr)) => {
                println!("{:?} bytes received from {:?}", len, client_addr);

                // Convert the received message into a string
                let received_message = String::from_utf8_lossy(&buf[..len]).to_string();

                // Emit the message to the Tauri frontend
                app_handle.emit("udp-message", received_message).unwrap_or_else(|e| {
                    eprintln!("Failed to emit UDP message: {}", e);
                });

                // Send data back to the sender
                if let Err(e) = sock.send_to(&buf[..len], client_addr).await {
                    eprintln!("Failed to send data to {:?}: {}", client_addr, e);
                } else {
                    println!("{:?} bytes sent", len);
                }
            }
            Err(e) => {
                eprintln!("Failed to receive data: {}", e);
                return Err(format!("Socket receive error: {}", e));
            }
        }
    }
}


