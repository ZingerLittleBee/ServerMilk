use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"));

    // 设置在右键单击系统托盘时显示菜单
    let tray = SystemTray::new()
        .with_menu(tray_menu);

    #[cfg(target_os = "macos")]
    // 左键点击不显示菜单
    return tray.with_menu_on_left_click(false);

    #[cfg(not(target_os = "macos"))]
    return tray;
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();

    match event {
        // 根据菜单 id 进行事件匹配
        SystemTrayEvent::MenuItemClick { id, .. } => if id.as_str() == "quit" {
            std::process::exit(0);
        },
        _ => {
            window.center().unwrap();
            window.show().unwrap();
            window.set_always_on_top(true).unwrap();
        }
    }
}
