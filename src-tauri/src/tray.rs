use tauri::api::dialog;
use tauri::async_runtime::spawn;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
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
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();

    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "show" => {
                window.show().unwrap();
                window.set_focus().unwrap();
            }
            "reload" => {
                window.app_handle().restart();
            }
            "log" => {
                let log_path = app.path_resolver().app_log_dir().map(| dir | dir.join("web.log"));
                if let Some(log_path) = log_path {
                    match open::that(log_path) {
                        Ok(_) => {},
                        Err(err) => {
                            dialog::message(
                                Some(&window),
                                "Open Log",
                                format!("Open log file failed: {}", err),
                            );
                        }
                    };
                } else {
                    dialog::message(
                        Some(&window),
                        "Open Log",
                        "Log file not exists",
                    );
                }
            }
            "devtool" => {
                window.open_devtools();
            }
            "autostart" => {
                let app_clone = app.clone();

                MessageDialogBuilder::new("Autostart", "Enable or Disable autostart when system startup")
                    .parent(&window)
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

            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    }
}
