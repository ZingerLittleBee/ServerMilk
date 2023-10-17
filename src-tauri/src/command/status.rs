use std::sync::{Arc, RwLock};
use crate::SidecarState;

#[tauri::command]
pub fn check_running_status(state: tauri::State<Arc<RwLock<SidecarState>>>) -> bool {
    if let Ok(state) = state.try_read() {
        state.get_pid() > 0
    } else {
        false
    }
}

#[tauri::command]
pub fn get_pid(state: tauri::State<Arc<RwLock<SidecarState>>>) -> u32 {
    if let Ok(state) = state.try_read() {
        state.get_pid()
    } else {
        0
    }
}
