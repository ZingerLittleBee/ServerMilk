#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use tauri::{Manager, WindowUrl};
use tauri::api::process::{Command, CommandEvent};
use tauri::utils::config::AppUrl;
use tauri_plugin_autostart::MacosLauncher;
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
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--flag1", "--flag2"])))
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

            let main_window = app.get_window("main").unwrap();

            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();

            main_window.set_transparent_titlebar(true);
            main_window.center().unwrap();

            #[cfg(target_os = "macos")]
            main_window.eval(r#"
                let newDiv = document.createElement('div');
                newDiv.setAttribute('data-tauri-drag-region', '');
                newDiv.style.height = '20px';
                newDiv.style.width = 'calc(100% - 70px)';
                newDiv.style.position = 'absolute';
                newDiv.style.top = '0';
                newDiv.style.marginLeft = '70px'
                newDiv.style.zIndex = '999';
                newDiv.style.cursor = 'move';
                document.body.prepend(newDiv);
            "#).unwrap();

            #[cfg(target_os = "macos")]
            main_window.eval(r#"
                window.addEventListener('load', (event) => {

                    function waitForElement(selector, callback) {
                      const element = document.querySelector(selector);

                      if(element) {
                        callback(element);
                        return;
                      }

                      setTimeout(() => waitForElement(selector, callback), 100);
                    }

                    waitForElement('header', header => {
                      header.style.paddingBottom = '10px';
                      let firstChild = header.firstElementChild;
                      firstChild.style.alignItems = 'end';
                    });
                });
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
