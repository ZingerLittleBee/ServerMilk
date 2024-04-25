use tauri::Manager;
use crate::constant::CONTROL_PANEL_WINDOW_LABEL;

#[tauri::command]
pub fn exit_command(handle: tauri::AppHandle) {
    handle.get_window(CONTROL_PANEL_WINDOW_LABEL).unwrap().hide().expect("hide window failed");
}
