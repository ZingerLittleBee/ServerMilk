use std::path::PathBuf;

#[tauri::command]
pub fn open_web_log(app_handle: tauri::AppHandle) {
    let log_path = log_path(app_handle);
    if log_path.exists() {
        let _ = open::that(log_path);
    }
}

#[tauri::command]
pub fn get_log_path(app_handle: tauri::AppHandle) -> String {
    app_handle
        .path_resolver()
        .app_log_dir()
        .unwrap().display().to_string()
}

fn log_path(app_handle: tauri::AppHandle) -> PathBuf {
    app_handle
        .path_resolver()
        .app_log_dir()
        .unwrap()
        .join("web.log")
}
