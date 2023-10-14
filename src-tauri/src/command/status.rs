use std::sync::{Arc, RwLock};
use crate::SidecarState;

#[tauri::command]
pub fn check_running_status(state: tauri::State<Arc<RwLock<SidecarState>>>) -> bool {
    if let Ok(state) = state.try_read() {
        if let Some(child) = &state.child {
            child.pid() > 0
        } else {
             false
        }
    } else {
        false
    }
}

#[tauri::command]
pub fn get_pid(state: tauri::State<Arc<RwLock<SidecarState>>>) -> u32 {
    if let Ok(state) = state.try_read() {
        if let Some(child) = &state.child {
            child.pid()
        } else {
            0
        }
    } else {
        0
    }
}
