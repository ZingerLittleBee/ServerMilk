use crate::state::SidecarState;
use std::sync::{Arc, RwLock};
use crate::window_manager::open_dashboard;

// When creating windows in a Tauri command, ensure the command function is async to avoid a deadlock on Windows due to the wry#583 issue.
// https://github.com/tauri-apps/wry/issues/583
#[cfg(not(target_os = "macos"))]
#[tauri::command]
pub async fn open_dashboard_command(handle: tauri::AppHandle, state: tauri::State<'_, Arc<RwLock<SidecarState>>>) -> Result<bool, String> {
    open_dashboard(handle, state)
}

#[cfg(target_os = "macos")]
#[tauri::command]
pub fn open_dashboard_command(handle: tauri::AppHandle, state: tauri::State<'_, Arc<RwLock<SidecarState>>>) -> Result<bool, String> {
    open_dashboard(handle, state)
}
