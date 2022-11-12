#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

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

use crate::command::{
    launch::{disable_auto_launch, enable_auto_launch, is_enable_auto_launch},
    log::open_web_log,
    port::is_free_port,
    state::web_server_restart,
};
use tauri::{LogicalSize, Manager, Size};

use crate::command::state::WebServerState;
use crate::command::status::check_web_status;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_enable_auto_launch,
            enable_auto_launch,
            disable_auto_launch,
            web_server_restart,
            open_web_log,
            is_free_port,
            check_web_status
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .setup(|app| {
            let config_path = app
                .path_resolver()
                .app_config_dir()
                .unwrap()
                .join("settings.json");
            let log_path = app.path_resolver().app_log_dir().unwrap().join("web.log");
            let (srv, sys) = runner::web_runner(config_path, log_path);
            let web_server_state = WebServerState::new((srv, sys));
            app.manage(web_server_state);

            let main_window = app.get_window("main").unwrap();
            main_window
                .set_size(Size::Logical(LogicalSize {
                    width: 320.0,
                    height: 360.0,
                }))
                .unwrap();
            main_window.set_resizable(false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
