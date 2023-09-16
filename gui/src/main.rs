#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cmd;
use cmd::*;

fn main() {
    println!("{}", gitauri::times(5, 9));
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
