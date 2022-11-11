use port_selector::is_free;

#[tauri::command]
pub fn is_free_port(port: u16) -> bool {
    is_free(port)
}
