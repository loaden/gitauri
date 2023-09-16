#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    gitauri::create_app(tauri::Builder::default()).run(|_app_handle, event| {
        match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        }
    });
}
