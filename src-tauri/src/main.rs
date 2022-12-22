#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env::current_exe;

use anyhow::anyhow;
use anyhow::Result;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use tauri::{LogicalSize, Manager, Size};


use crate::command::{
    launch::{disable_auto_launch, enable_auto_launch, is_enable_auto_launch},
    log::{open_web_log, get_log_path},
    port::is_free_port,
    state::web_server_restart,
};
use crate::command::status::check_web_status;

mod command;
mod config;
mod dto;
mod model;
mod runner;
mod server;
mod system_info;
mod tray;
// mod user_handler;
mod vo;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_enable_auto_launch,
            enable_auto_launch,
            disable_auto_launch,
            web_server_restart,
            open_web_log,
            is_free_port,
            check_web_status,
            get_log_path
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
            event.window().hide().unwrap();
            api.prevent_close();
        })
        .setup(|app| {
            // don't show on the taskbar/springboard
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // let config_path = app
            //     .path_resolver()
            //     .app_config_dir()
            //     .unwrap()
            //     .join("settings.json");
            // let log_path = app.path_resolver().app_log_dir().unwrap().join("web.log");
            // let (srv, sys) = runner::web_runner(config_path, log_path);
            // let web_server_state = WebServerState::new((srv, sys));
            // app.manage(web_server_state);

            app.manage(init_launch().unwrap());

            let main_window = app.get_window("main").unwrap();

            main_window
                .set_size(Size::Logical(LogicalSize {
                    width: 320.0,
                    height: 360.0,
                }))
                .unwrap();
            main_window.center().unwrap();
            main_window.set_resizable(false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init_launch() -> Result<AutoLaunch> {
    let app_exe = current_exe()?;
    let app_exe = dunce::canonicalize(app_exe)?;
    let app_name = app_exe
        .file_stem()
        .and_then(|f| f.to_str())
        .ok_or_else(|| anyhow!("failed to get file stem"))?;

    let app_path = app_exe
        .as_os_str()
        .to_str()
        .ok_or_else(|| anyhow!("failed to get app_path"))?
        .to_string();

    #[cfg(target_os = "windows")]
        let app_path = format!("\"{app_path}\"");

    #[cfg(target_os = "macos")]
        let app_path = (|| -> Option<String> {
        let path = std::path::PathBuf::from(&app_path);
        let path = path.parent()?.parent()?.parent()?;
        let extension = path.extension()?.to_str()?;
        match extension == "app" {
            true => Some(path.as_os_str().to_str()?.to_string()),
            false => None,
        }
    })()
        .unwrap_or(app_path);

    Ok(AutoLaunchBuilder::new()
        .set_app_name(app_name)
        .set_app_path(&app_path)
        .build()?)
}
