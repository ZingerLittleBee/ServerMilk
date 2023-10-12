use tauri::api::dialog;
use tauri_plugin_autostart::ManagerExt;

#[tauri::command]
pub fn enable_auto_start(app_handle: tauri::AppHandle, window: tauri::Window) {
    match app_handle.autolaunch().enable() {
        Ok(_) => {
            dialog::message(
                Some(&window),
                "Success",
                "Auto start enabled",
            );
        },
        Err(err) => {
            dialog::message(
                Some(&window),
                "Failed",
                format!("Enable auto start failed: {}", err),
            );
        }
    }
}

#[tauri::command]
pub fn disable_auto_start(app_handle: tauri::AppHandle, window: tauri::Window) {
    match app_handle.autolaunch().disable() {
        Ok(_) => {
            dialog::message(
                Some(&window),
                "Success",
                "Auto start disabled",
            );
        },
        Err(err) => {
            dialog::message(
                Some(&window),
                "Failed",
                format!("Disable auto start failed: {}", err),
            );
        }
    }
}
