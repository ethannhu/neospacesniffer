// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::ipc::Response;

mod sniff;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn fetch_files() -> Response {
    let data = std::fs::read("files.json").unwrap();
    Response::new(data)
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, fetch_files, sniff::recursive_search])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
