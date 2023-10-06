#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env::current_exe;

use anyhow::anyhow;
use anyhow::Result;
use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use tauri::{LogicalSize, Size, WindowBuilder, WindowUrl};
use tauri::api::process::{Command, CommandEvent};
use tauri::utils::config::AppUrl;
use window_shadows::set_shadow;


use crate::command::{
    launch::{disable_auto_launch, enable_auto_launch, is_enable_auto_launch},
    log::{open_web_log, get_log_path},
    port::is_free_port,
};
use crate::command::status::check_web_status;
use crate::ext::window::WindowExt;

mod command;
mod tray;
mod ext;

fn main() {
    let (mut rx, mut child) = Command::new_sidecar("serverbee-web")
        .expect("failed to create `serverbee-web` binary command")
        .args(["--port", "9527"])
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
        .invoke_handler(tauri::generate_handler![
            is_enable_auto_launch,
            enable_auto_launch,
            disable_auto_launch,
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
        .setup(move |app| {
            // don't show on the taskbar/springboard
            // #[cfg(target_os = "macos")]
            // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // app.manage(init_launch().unwrap());

            let main_window = WindowBuilder::new(
                app,
                "main".to_string(),
                if cfg!(dev) {
                    Default::default()
                } else {
                    window_url
                }
            )
                .build()?;

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();

            main_window.set_transparent_titlebar(true);
            main_window.set_size(Size::Logical(LogicalSize {
                width: 1400.0,
                height: 842.0,
            })).unwrap();
            main_window.set_min_size(
                Some(Size::Logical(LogicalSize {
                    width: 400.0,
                    height: 200.0,
                }))
            ).unwrap();
            main_window.center().unwrap();

            #[cfg(target_os = "macos")]
            main_window.eval(r#"
                let newDiv = document.createElement('div');
                newDiv.setAttribute('data-tauri-drag-region', '');
                newDiv.style.height = '15px';
                newDiv.style.width = '100%';
                newDiv.style.position = 'absolute';
                newDiv.style.top = '0';
                newDiv.style.zIndex = '999';
                document.body.prepend(newDiv);
            "#).unwrap();

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
