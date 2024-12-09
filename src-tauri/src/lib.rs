use std::sync::Mutex;

use app_state::AppState;
use tauri::Manager;

mod app_state;
mod server;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(state: tauri::State<'_,Mutex<AppState>>, name: &str) -> String {
    let mut state = state.lock().unwrap();
    state.add_name(name.to_string());
    let len = state.names.len();
    let formatted_names = state.names.join(", ");
    format!("Hello, these are the names that have been saved {}. Length: {}", formatted_names, len)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
    .setup(|app| {
        app.manage(Mutex::new(AppState::new()));
        Ok(())
    })
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet, server::create_server, server::get_servers])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


