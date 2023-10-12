#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::command::dialog::open_message_dialog;
use crate::command::log::open_log;
use crate::command::status::{check_running_status, get_pid};
use crate::command::auto_start::{enable_auto_start, disable_auto_start};
use ::log::info;
use std::sync::Mutex;
use tauri::api::process::{Command, CommandChild};
use tauri::{LogicalSize, Manager};
use tauri_plugin_autostart::MacosLauncher;

#[cfg(target_os = "macos")]
use crate::ext::window::WindowExt;
use crate::shortcut::register_shortcut;

mod command;
mod constant;
mod dashboard;
mod ext;
mod hacker;
mod logs;
mod shortcut;
mod tray;
mod utils;

pub struct SidecarState {
    child: Mutex<Option<CommandChild>>,
}

fn main() {
    // make sure ../dist exists
    // let mut context = tauri::generate_context!();
    // let url = format!("http://localhost:{}", 9527).parse().unwrap();
    // let window_url = WindowUrl::External(url);
    // // rewrite the config so the IPC is enabled on this URL
    // context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    // context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .manage(SidecarState {
            child: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            open_log,
            open_message_dialog,
            check_running_status,
            get_pid,
            enable_auto_start,
            disable_auto_start,
        ])
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .setup(move |app| {
            // don't show on the taskbar/springboard
            // #[cfg(target_os = "macos")]
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let log_dir = app
                .path_resolver()
                .app_log_dir()
                .expect("failed to get log dir");
            let data_dir = app
                .path_resolver()
                .app_data_dir()
                .expect("failed to get data dir");

            logs::init_log(&log_dir);

            let cmd_args: Vec<String> = vec![
                "--port".into(),
                "9527".into(),
                "-l".into(),
                log_dir.to_str().unwrap().into(),
                "-d".into(),
                data_dir.to_str().unwrap().into(),
            ];

            let (_rx, child) = Command::new_sidecar("serverbee-web")
                .expect("failed to create `serverbee-web` binary command")
                .args(cmd_args)
                .spawn()
                .expect("Failed to spawn sidecar");

            info!("child pid: {:?}", child.pid());

            let state = app.state::<SidecarState>();
            let mut child_lock = state.child.lock().unwrap();
            *child_lock = Some(child);

            // let output = Command::new_sidecar("serverbee-web")
            //     .expect("failed to create `serverbee-web` binary command")
            //     .args(cmd_args)
            //     .output()
            //     .expect("Failed to spawn sidecar");

            let main_window = app.get_window("main").unwrap();
            // main_window.hide().unwrap();
            main_window.set_title("Settings").unwrap();
            main_window
                .set_size(LogicalSize {
                    width: 500.0,
                    height: 420.0,
                })
                .unwrap();
            main_window.set_maximizable(false).unwrap();
            main_window.set_minimizable(false).unwrap();

            #[cfg(not(target_os = "macos"))]
            main_window.set_decorations(false).unwrap();

            // open_dashboard(app.handle());

            register_shortcut(app.handle());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
