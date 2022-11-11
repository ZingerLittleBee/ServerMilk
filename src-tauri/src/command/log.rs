#[tauri::command]
pub fn open_web_log(app_handle: tauri::AppHandle) {
    let log_path = app_handle
        .path_resolver()
        .app_log_dir()
        .unwrap()
        .join("web.log");
    if log_path.exists() {
        let _ = open::that(log_path);
    }
}
