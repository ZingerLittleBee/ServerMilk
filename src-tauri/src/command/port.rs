use port_selector::is_free;
use crate::SidecarState;

#[tauri::command]
pub fn is_free_port(port: u16) -> bool {
    is_free(port)
}


#[tauri::command]
pub fn get_port(state: tauri::State<SidecarState>) -> u16 {
    let port_lock = state.port.lock().unwrap();
    port_lock.unwrap()
}
