pub fn create_app<R: tauri::Runtime>(
    builder: tauri::Builder<R>,
) -> tauri::App<R> {
    builder
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    if name.len() == 0 || name.trim().len() == 0 {
        return "Please input your name!".to_string();
    }
    format!(
        "Hello, {}! You've been greeted from {} {}!",
        name,
        base::G_APPNAME,
        base::add(1, 2) + times(2, 3)
    )
}

pub fn times(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = times(2, 3);
        assert_eq!(result, 6);
    }
}
