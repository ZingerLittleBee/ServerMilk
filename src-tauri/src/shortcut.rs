use tauri::{GlobalShortcutManager, Manager};

pub fn register_shortcut(handle: tauri::AppHandle) {
    let mut manager = handle.global_shortcut_manager();
    manager.register(
        "CmdOrCtrl+,",
        move || {
            let settings_window = handle.get_window("main").unwrap();
            settings_window.show().unwrap();
        },
    ).unwrap();
}
