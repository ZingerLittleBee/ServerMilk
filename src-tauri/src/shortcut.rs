use log::error;
use tauri::{GlobalShortcutManager, Manager};
use crate::constant::DEFAULT_WINDOW_LABEL;

pub fn register_shortcut(handle: tauri::AppHandle) {
    let mut manager = handle.global_shortcut_manager();
    match manager.register(
        "CmdOrCtrl+,",
        move || {
            if let Some(settings_window) = handle.get_window(DEFAULT_WINDOW_LABEL) {
                match settings_window.show() {
                    Ok(_) => {},
                    Err(e) => error!("error showing settings window while registering shortcut: {}", e),
                }
            } else {
                error!("settings window not found while registering shortcut");
            }
        },
    ) {
        Ok(_) => {},
        Err(e) => error!("error registering shortcut: {}", e),
    }
}
