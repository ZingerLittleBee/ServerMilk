use std::sync::{Arc, RwLock};
use log::info;
use tauri::api::process::Command;
use crate::constant::DEFAULT_PORT;
use crate::SidecarState;

#[tauri::command]
pub fn start_sidecar(app_handle: tauri::AppHandle, state: tauri::State<Arc<RwLock<SidecarState>>>, port: Option<u16>) {
    let log_dir = app_handle
        .path_resolver()
        .app_log_dir()
        .expect("failed to get log dir");
    let data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .expect("failed to get data dir");

    let port = port.unwrap_or_else(|| {
        match state.try_read() {
            Ok(state) => {
                state.get_port()
            }
            Err(_) => {
                DEFAULT_PORT
            }
        }
    });

    info!("start sidecar with port: {}", port);

    match state.try_write() {
        Ok(mut state) => {
            state.set_port(port).unwrap();
        }
        Err(_) => {
            info!("failed to set port");
        }
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

    match state.try_write() {
        Ok(mut state) => {
            state.child = Some(child);
        }
        Err(_) => {
            info!("failed to set child");
        }
    }
}

#[tauri::command]
pub fn kill_sidecar(state: tauri::State<Arc<RwLock<SidecarState>>>) -> bool {
    if let Ok(mut state) = state.try_write() {
        state.kill_sidecar()
    } else {
        false
    }
}

#[tauri::command]
pub fn restart_sidecar(app_handle: tauri::AppHandle, state: tauri::State<Arc<RwLock<SidecarState>>>) {
    kill_sidecar(state.clone());
    start_sidecar(app_handle, state, None);
}

#[tauri::command]
pub fn start_with_new_port(app_handle: tauri::AppHandle, state: tauri::State<Arc<RwLock<SidecarState>>>, port: u16) {
    kill_sidecar(state.clone());
    start_sidecar(app_handle, state, Some(port));
}
