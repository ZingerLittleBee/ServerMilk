#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env::current_exe;

use auto_launch::AutoLaunchBuilder;
use tauri::{LogicalSize, Manager, Size};

mod command;
mod config;
mod model;
mod runner;
mod server;
mod system_info;
mod tray;
mod vo;

fn main() {
    tauri::async_runtime::spawn(async move {
        let rt = tokio::runtime::Handle::current();
        let rt_ = rt.clone();
        rt.spawn_blocking(move || {
            rt_.block_on(async {
                let local = tokio::task::LocalSet::new();
                let _ = local.run_until(runner::web_runner()).await;
            })
        });
    });

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![command::is_enable_auto_launch,
            command::enable_auto_launch,
            command::disable_auto_launch,
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window
                .set_size(Size::Logical(LogicalSize {
                    width: 350.0,
                    height: 400.0,
                }))
                .unwrap();
            main_window.set_resizable(false).unwrap();
            Ok(())
        }
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
