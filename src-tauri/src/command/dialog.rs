use tauri::api::dialog;
use tauri::Manager;

#[tauri::command]
pub fn open_message_dialog(app_handle: tauri::AppHandle,
                           title: String,
                           message: String,
                            windows: String
) {
    dialog::message(
        Some(&app_handle.get_window(windows.as_str()).unwrap()),
        title,
        message
    )
}
