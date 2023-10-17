use std::sync::{Arc, RwLock};
use port_selector::is_free;
use crate::SidecarState;
use crate::utils::get_port_from_state;

#[tauri::command]
pub fn is_free_port(port: u16) -> bool {
    is_free(port)
}


#[tauri::command]
pub fn get_port(state: tauri::State<Arc<RwLock<SidecarState>>>) -> u16 {
    get_port_from_state(state.clone())
}
