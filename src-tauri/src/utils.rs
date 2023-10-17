use std::sync::{Arc, RwLock};
use tauri::api::dialog;
use tauri::{AppHandle, Window};
use crate::constant::DEFAULT_PORT;
use crate::state::SidecarState;

pub fn open_web_log(app_handle: &AppHandle, window: &Window) {
    let log_path = app_handle.path_resolver().app_log_dir().map(| dir | dir.join("web.log"));
    if let Some(log_path) = log_path {
        match open::that(log_path) {
            Ok(_) => {},
            Err(err) => {
                dialog::message(
                    Some(window),
                    "Open Logs",
                    format!("Open log file failed: {}", err),
                );
            }
        };
    } else {
        dialog::message(
            Some(window),
            "Open Logs",
            "Log file not exists",
        );
    }
}

pub fn get_port_from_state(state: tauri::State<'_, Arc<RwLock<SidecarState>>>) -> u16 {
    let state = state.try_read();
    if let Ok(state) = state {
        state.get_port()
    } else {
        DEFAULT_PORT
    }
}
