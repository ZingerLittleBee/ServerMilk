use crate::SidecarState;

#[tauri::command]
pub fn check_running_status(state: tauri::State<SidecarState>) -> bool {
    let mut child = state.child.lock().unwrap();
    if child.is_none() {
        return false;
    }
    let child = child.as_mut().unwrap();
    child.pid() > 0
}

#[tauri::command]
pub fn get_pid(state: tauri::State<SidecarState>) -> u32 {
    let mut child = state.child.lock().unwrap();
    if child.is_none() {
        return 0;
    }
    let child = child.as_mut().unwrap();
    child.pid()
}
