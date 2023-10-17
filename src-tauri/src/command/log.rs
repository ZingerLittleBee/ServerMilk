use crate::utils::open_web_log;

#[tauri::command]
pub fn open_log(app_handle: tauri::AppHandle, window: tauri::Window) {
    open_web_log(
        &app_handle,
        &window,)
}
