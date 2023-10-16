use std::sync::{Arc, RwLock};
use crate::utils::open_web_log;
use tauri::api::dialog;
use tauri::async_runtime::spawn;
use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};
use crate::command::dashboard::open_dashboard;
use crate::state::SidecarState;

pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new(
            "open_control_panel".to_string(),
            "Open Control Panel",
        ))
        .add_item(CustomMenuItem::new(
            "open_dashboard".to_string(),
            "Open Dashboard",
        ))
        .add_item(CustomMenuItem::new("reload", "Reload Dashboard"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("log", "Open Log"))
        .add_item(CustomMenuItem::new("devtool".to_string(), "Open DevTool"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("update".to_string(), "Check Update"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(
            CustomMenuItem::new("settings".to_string(), "Settings").accelerator("CmdOrCtrl+,"),
        )
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit").accelerator("CmdOrCtrl+Q"));

    SystemTray::new().with_menu(tray_menu)
}

pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let control_panel_window = app.get_window("main").unwrap();
    let dashboard_window_option = app.get_window("dashboard");

    if let SystemTrayEvent::MenuItemClick { id, .. } = event {
        match id.as_str() {
            "open_control_panel" => {
                control_panel_window.show().unwrap();
                control_panel_window.set_focus().unwrap();
            }
            "open_dashboard" => match dashboard_window_option {
                None => {
                    open_dashboard(app.clone(), app.state::<Arc<RwLock<SidecarState>>>());
                }
                Some(dashboard_window) => {
                    dashboard_window.show().unwrap();
                    dashboard_window.set_focus().unwrap();
                }
            },
            "reload" => match dashboard_window_option {
                None => {}
                Some(dashboard_window) => dashboard_window.app_handle().restart(),
            },
            "log" => match dashboard_window_option {
                None => {}
                Some(dashboard_window) => {
                    open_web_log(&dashboard_window.app_handle(), &dashboard_window)
                }
            },
            "devtool" => match dashboard_window_option {
                None => {}
                Some(dashboard_window) => dashboard_window.open_devtools(),
            },
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
                control_panel_window.show().unwrap();
                control_panel_window.center().unwrap();
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        }
    }
}
