use log::info;
use tauri::api::process::Command;
use crate::constant::DEFAULT_PORT;
use crate::SidecarState;

#[tauri::command]
pub fn start_sidecar(app_handle: tauri::AppHandle, state: tauri::State<SidecarState>, port: Option<u16>) {
    let log_dir = app_handle
        .path_resolver()
        .app_log_dir()
        .expect("failed to get log dir");
    let data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("failed to get data dir");

    let port = port.unwrap_or_else(|| {
        let store = state.store.lock().unwrap();
        store.as_ref().unwrap()
            .get("port")
            .map(|v| v.as_u64().unwrap() as u16)
            .unwrap_or(DEFAULT_PORT)
    });

    info!("port: {}", port);

    {
        let mut port_lock = state.port.lock().unwrap();
        *port_lock = Some(port);
    }

    let cmd_args: Vec<String> = vec![
        "--port".into(),
        port.to_string(),
        "-l".into(),
        log_dir.to_str().unwrap().into(),
        "-d".into(),
        data_dir.to_str().unwrap().into(),
    ];

    info!("cmd_args: {:?}", cmd_args);

    let (_rx, child) = Command::new_sidecar("serverbee-web")
        .expect("failed to create `serverbee-web` binary command")
        .args(cmd_args)
        .spawn()
        .expect("Failed to spawn sidecar");

    info!("child pid: {:?}", child.pid());
    let mut child_lock = state.child.lock().unwrap();
    *child_lock = Some(child);
}

#[tauri::command]
pub fn kill_sidecar(state: tauri::State<SidecarState>) -> bool {
    let mut child_lock = state.child.lock().unwrap();
    if let Some(child) = child_lock.take() {
        child.kill().is_ok()
    } else {
        false
    }
}

#[tauri::command]
pub fn restart_sidecar(app_handle: tauri::AppHandle, state: tauri::State<SidecarState>) {
    kill_sidecar(state.clone());
    start_sidecar(app_handle, state, None);
}

#[tauri::command]
pub fn start_with_new_port(app_handle: tauri::AppHandle, state: tauri::State<SidecarState>, port: u16) {
    kill_sidecar(state.clone());
    start_sidecar(app_handle, state, Some(port));
}
