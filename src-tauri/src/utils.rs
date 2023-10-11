use tauri::api::dialog;
use tauri::{AppHandle, Window};

pub fn open_web_log(app_handle: &AppHandle, window: &Window) {
    let log_path = app_handle.path_resolver().app_log_dir().map(| dir | dir.join("web.log"));
    if let Some(log_path) = log_path {
        match open::that(log_path) {
            Ok(_) => {},
            Err(err) => {
                dialog::message(
                    Some(window),
                    "Open Log",
                    format!("Open log file failed: {}", err),
                );
            }
        };
    } else {
        dialog::message(
            Some(window),
            "Open Log",
            "Log file not exists",
        );
    }
}
