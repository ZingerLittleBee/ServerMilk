use std::sync::{Arc, RwLock};
use port_selector::is_free;
use crate::constant::DEFAULT_PORT;
use crate::SidecarState;

#[tauri::command]
pub fn is_free_port(port: u16) -> bool {
    is_free(port)
}


#[tauri::command]
pub fn get_port(state: tauri::State<Arc<RwLock<SidecarState>>>) -> u16 {
    match state.try_read() {
        Ok(state) => {
            state.get_port()
        }
        Err(_) => {
            DEFAULT_PORT
        }
    }
}
