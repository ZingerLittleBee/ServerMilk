use tauri::api::dialog;
use tauri::async_runtime::spawn;
use tauri::{AppHandle, CustomMenuItem, GlobalShortcutManager, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use tauri::api::dialog::{MessageDialogBuilder, MessageDialogButtons};
use tauri_plugin_autostart::ManagerExt;

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "Open ServerMilk"))
        .add_item(CustomMenuItem::new("reload", "Reload"))
        .add_item(CustomMenuItem::new("log", "Open Log"))

        .add_item(CustomMenuItem::new("devtool".to_string(), "Open DevTool"))
        .add_item(CustomMenuItem::new("autostart".to_string(), "Autostart"))
        .add_item(CustomMenuItem::new("update".to_string(), "Check Update"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("settings".to_string(), "Settings").accelerator("CmdOrCtrl+,"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let dashboard_window = app.get_window("dashboard").unwrap();

    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "show" => {
                dashboard_window.show().unwrap();
                dashboard_window.set_focus().unwrap();
            }
            "reload" => {
                dashboard_window.app_handle().restart();
            }
            "log" => {
                let log_path = app.path_resolver().app_log_dir().map(| dir | dir.join("web.log"));
                if let Some(log_path) = log_path {
                    match open::that(log_path) {
                        Ok(_) => {},
                        Err(err) => {
                            dialog::message(
                                Some(&dashboard_window),
                                "Open Log",
                                format!("Open log file failed: {}", err),
                            );
                        }
                    };
                } else {
                    dialog::message(
                        Some(&dashboard_window),
                        "Open Log",
                        "Log file not exists",
                    );
                }
            }
            "devtool" => {
                dashboard_window.open_devtools();
            }
            "autostart" => {
                let app_clone = app.clone();

                MessageDialogBuilder::new("Autostart", "Enable or Disable autostart when system startup")
                    .parent(&dashboard_window)
                    .buttons(MessageDialogButtons::OkCancelWithLabels(
                        "Enable".into(),
                        "Disable".into(),
                    ))
                    .show(move |ok| {
                        if ok {
                            app_clone.autolaunch().enable().unwrap();
                        }
                        else {
                            app_clone.autolaunch().disable().unwrap();
                        }
                    });
            }
            "update" => {
                let app_clone = app.clone();

                spawn(async move {
                    let res = app_clone.updater().check().await.unwrap();

                    dialog::message(
                        Some(&app_clone.get_window("main").unwrap()),
                        "Update",
                        if res.is_update_available() {
                            "Update available"
                        } else {
                            "No update available"
                        },
                    )
                });
            }
            "settings" => {
                app.get_window("main").unwrap().show().unwrap();
                app.get_window("main").unwrap().center().unwrap();

                match app.app_handle().global_shortcut_manager().is_registered("CmdOrCtrl+Q").unwrap() {
                    true => {
                        println!("set resizable");
                        app.get_window("main").unwrap().set_resizable(true).unwrap();
                    }
                    false => {
                        println!("set not resizable");
                        app.get_window("main").unwrap().set_resizable(false).unwrap();
                    }
                }

            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    }
}
