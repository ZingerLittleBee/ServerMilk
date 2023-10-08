#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::api::process::{Command, CommandEvent};
use tauri::utils::config::AppUrl;
use tauri::{Manager, WindowUrl};
use tauri_plugin_autostart::MacosLauncher;
use window_shadows::set_shadow;

#[cfg(target_os = "macos")]
use crate::ext::window::WindowExt;
use crate::utils::app_log_dir;

mod command;
mod ext;
mod hacker;
mod tray;
mod utils;

fn main() {
    let app_log_dir = app_log_dir();

    let mut cmd_args: Vec<String> = vec!["--port".into(), "9527".into()];

    if let Some(log_dir) = app_log_dir {
        let log_dir = log_dir.to_str();
        if let Some(log_dir) = log_dir {
            cmd_args.push("-l".into());
            cmd_args.push(log_dir.to_string());
        }
    }
    let (mut rx, mut child) = Command::new_sidecar("serverbee-web")
        .expect("failed to create `serverbee-web` binary command")
        .args(cmd_args)
        .spawn()
        .expect("Failed to spawn sidecar");

    // make sure ../dist exists
    let mut context = tauri::generate_context!();
    let url = format!("http://localhost:{}", 9527).parse().unwrap();
    let window_url = WindowUrl::External(url);
    // rewrite the config so the IPC is enabled on this URL
    context.config_mut().build.dist_dir = AppUrl::Url(window_url.clone());
    context.config_mut().build.dev_path = AppUrl::Url(window_url.clone());

    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
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

            let main_window = app.get_window("main").unwrap();

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();

            #[cfg(target_os = "macos")]
            main_window.set_transparent_titlebar(true);

            #[cfg(not(target_os = "macos"))]
            main_window.set_decorations(false).unwrap();

            main_window.center().unwrap();

            main_window.eval(hacker::CRATE_DRAG_REGION).unwrap();

            main_window.eval(hacker::MODIFY_NAVBAR).unwrap();

            tauri::async_runtime::spawn(async move {
                // read events such as stdout
                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        main_window
                            .emit("message", Some(format!("'{}'", line)))
                            .expect("failed to emit event");
                        // write to stdin
                        child.write("message from Rust\n".as_bytes()).unwrap();
                    }
                }
            });

            Ok(())
        })
        .run(context)
        .expect("error while running tauri application");
}
