use crate::constant::DASHBOARD_WINDOW_LABEL;
#[cfg(target_os = "macos")]
use crate::ext::window::WindowExt;
use crate::hacker;
use crate::state::SidecarState;
use crate::utils::get_port_from_state;
use log::{debug, error};
use std::sync::{Arc, RwLock};
use tauri::{LogicalSize, Manager};
use window_shadows::set_shadow;

// When creating windows in a Tauri command, ensure the command function is async to avoid a deadlock on Windows due to the wry#583 issue.
// https://github.com/tauri-apps/wry/issues/583
#[tauri::command]
pub async fn open_dashboard_command(handle: tauri::AppHandle, state: tauri::State<'_, Arc<RwLock<SidecarState>>>) -> Result<bool, String> {
    return open_dashboard(handle, state);
}


pub fn open_dashboard(handle: tauri::AppHandle, state: tauri::State<Arc<RwLock<SidecarState>>>) -> Result<bool, String> {
    if let Some(window) = handle.get_window(DASHBOARD_WINDOW_LABEL) {
        debug!("dashboard window is already opened");
        window.show().unwrap();
        window.set_focus().unwrap();
        return Ok(true);
    }

    let port = get_port_from_state(state.clone());

    let url = match format!("http://localhost:{}", port).parse() {
        Ok(url) => url,
        Err(e) => {
            error!("failed to parse url: {}", e);
            return Ok(false);
        }
    };

    debug!("dashboard url: {}", url);

    let dashboard_window = match tauri::WindowBuilder::new(
        &handle,
        DASHBOARD_WINDOW_LABEL,
        tauri::WindowUrl::External(url),
    )
    .build()
    {
        Ok(window) => window,
        Err(e) => {
            error!("failed to create dashboard window: {}", e);
            return Ok(false);
        }
    };

    match dashboard_window.set_title("Dashboard") {
        Ok(_) => {}
        Err(e) => error!("failed to set dashboard window title: {}", e),
    }

    #[cfg(target_os = "macos")]
    match dashboard_window.set_decorations(true) {
        Ok(_) => {}
        Err(e) => error!("failed to set dashboard window decorations: {}", e),
    }

    #[cfg(not(target_os = "macos"))]
    match dashboard_window.set_decorations(false) {
        Ok(_) => {}
        Err(e) => error!("failed to set dashboard window decorations: {}", e),
    }

    match dashboard_window.set_size(LogicalSize {
        width: 1400.0,
        height: 890.0,
    }) {
        Ok(_) => {}
        Err(e) => error!("failed to set dashboard window size: {}", e),
    }

    match dashboard_window
        .set_min_size(Some(LogicalSize {
            width: 450.0,
            height: 200.0,
        })) {
            Ok(_) => {}
            Err(e) => error!("failed to set dashboard window min size: {}", e),
        }

    #[cfg(any(windows, target_os = "macos"))]
    match set_shadow(&dashboard_window, true) {
        Ok(_) => {},
        Err(e) => error!("failed to set shadow: {}", e),
    }

    #[cfg(target_os = "macos")]
    dashboard_window.set_transparent_titlebar(true);

    match dashboard_window.eval(hacker::CRATE_DRAG_REGION) {
        Ok(_) => {}
        Err(e) => error!("failed to set drag region: {}", e),
    }
    match dashboard_window.eval(hacker::MODIFY_NAVBAR) {
        Ok(_) => {}
        Err(e) => error!("failed to modify navbar: {}", e),
    }
    Ok(true)
}