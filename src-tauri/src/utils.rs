use std::path::PathBuf;

pub fn app_log_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
        let path = tauri::api::path::home_dir().map(|dir| {
        dir
            .join("Library/Logs")
            .join("app.serverbee")
    });

    #[cfg(not(target_os = "macos"))]
        let path =
        tauri::api::path::config_dir().map(|dir| dir.join("app.serverbee").join("logs"));

    path
}
