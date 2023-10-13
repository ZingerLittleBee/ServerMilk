#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::command::auto_start::{disable_auto_start, enable_auto_start};
use crate::command::dialog::open_message_dialog;
use crate::command::log::open_log;
use crate::command::status::{check_running_status, get_pid};
use crate::command::port::{get_port, is_free_port};
use crate::constant::DEFAULT_PORT;
use ::log::info;
use port_selector::{select_from_given_port};
use std::sync::Mutex;
use tauri::api::process::{Command, CommandChild};
use tauri::{LogicalSize, Manager, Wry};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_store::{Store, StoreBuilder};

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
    store: Mutex<Option<Store<Wry>>>,
    port: Mutex<Option<u16>>,
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
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .manage(SidecarState {
            child: Mutex::new(None),
            store: Mutex::new(None),
            port: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            open_log,
            open_message_dialog,
            check_running_status,
            get_pid,
            get_port,
            is_free_port,
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

            let config_dir = app
                .path_resolver()
                .app_config_dir()
                .expect("failed to get config dir");

            let state = app.state::<SidecarState>();

            let store = StoreBuilder::new(app.handle(), config_dir).build();

            let port = store
                .get("port")
                .map(|v| select_from_given_port(v.as_u64().unwrap() as u16).unwrap())
                .unwrap_or_else(|| select_from_given_port(DEFAULT_PORT).unwrap());

            let mut store_lock = state.store.lock().unwrap();
            *store_lock = Some(store);

            let mut port_lock = state.port.lock().unwrap();
            *port_lock = Some(port);

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
                port.to_string(),
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


            let mut child_lock = state.child.lock().unwrap();
            *child_lock = Some(child);

            let main_window = app.get_window("main").unwrap();
            // main_window.hide().unwrap();
            main_window.set_title("Control Panel").unwrap();
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
