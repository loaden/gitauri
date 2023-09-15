use gitauri::times;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!(
        "Hello, {}! You've been greeted from {} {}!",
        name,
        base::G_APPNAME,
        base::add(1, 2) + times(2, 3)
    )
}
