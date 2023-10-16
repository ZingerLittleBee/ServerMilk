#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::command::auto_start::{disable_auto_start, enable_auto_start, is_enable_auto_start};
use crate::command::dialog::open_message_dialog;
use crate::command::log::open_log;
use crate::command::port::{get_port, is_free_port};
use crate::command::sidecar::{restart_sidecar, start_sidecar, start_with_new_port};
use crate::command::status::{check_running_status, get_pid};
use crate::command::token::{fetch_token, set_token};
use crate::constant::SETTINGS_FILE_NAME;
use log::{info, warn};
use std::sync::{Arc, RwLock};
use tauri::{LogicalSize, Manager};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_store::StoreBuilder;

#[cfg(target_os = "macos")]
use crate::ext::window::WindowExt;
use crate::shortcut::register_shortcut;
use crate::state::SidecarState;

mod command;
mod constant;
mod ext;
mod hacker;
mod logs;
mod shortcut;
mod state;
mod tray;
mod utils;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![]),
        ))
        .invoke_handler(tauri::generate_handler![
            fetch_token,
            set_token,
            open_log,
            open_message_dialog,
            check_running_status,
            get_pid,
            get_port,
            is_free_port,
            is_enable_auto_start,
            enable_auto_start,
            disable_auto_start,
            restart_sidecar,
            start_with_new_port
        ])
        .manage(Arc::new(RwLock::new(SidecarState::default())))
        .system_tray(tray::menu())
        .on_system_tray_event(tray::handler)
        .on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event.event() {
                event.window().hide().unwrap();
                api.prevent_close();
            }
        })
        .setup(move |app| {
            let log_dir = app
                .path_resolver()
                .app_log_dir()
                .expect("failed to get log dir");

            logs::init_log(&log_dir);

            let config_dir = app
                .path_resolver()
                .app_config_dir()
                .expect("failed to get config dir");

            info!("config_dir: {:?}", config_dir);

            let state = app.state::<Arc<RwLock<SidecarState>>>();

            let mut store =
                StoreBuilder::new(app.handle(), config_dir.join(SETTINGS_FILE_NAME)).build();
            match store.load() {
                Ok(_) => {}
                Err(e) => {
                    warn!("failed to load store: {}", e);
                }
            }

            if let Ok(mut state) = state.try_write() {
                state.set_store(store);
            }

            start_sidecar(app.handle(), state.clone(), None);

            let main_window = app.get_window("main").unwrap();
            // main_window.hide().unwrap();
            main_window.set_title("Control Panel").unwrap();
            main_window
                .set_size(LogicalSize {
                    width: 420.0,
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
