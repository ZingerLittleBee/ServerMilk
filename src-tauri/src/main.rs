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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_enable_auto_launch,
            enable_auto_launch,
            disable_auto_launch,
            web_server_restart,
            open_web_log,
            is_free_port
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

            // async_runtime::spawn(async move {
            //     let rt = tokio::runtime::Handle::current();
            //     let rt_ = rt.clone();
            //     rt.spawn_blocking(move || {
            //         rt_.block_on(async {
            //             let local = tokio::task::LocalSet::new();
            //             let _ = local.run_until(runner::web_runner(config_dir)).await;
            //             println!("block on: ");
            //         })
            //     });
            // });

            let main_window = app.get_window("main").unwrap();
            main_window
                .set_size(Size::Logical(LogicalSize {
                    width: 350.0,
                    height: 400.0,
                }))
                .unwrap();
            main_window.set_resizable(false).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
