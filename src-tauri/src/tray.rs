use tauri::{
    api::dialog::message, AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent,
    SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    let tray_menu = SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            // 子菜单
            "File", // 子菜单名称
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("new_file".to_string(), "New File")) // 子菜单项（新增）
                .add_item(CustomMenuItem::new("edit_file".to_string(), "Edit File")), // 子菜单项（编辑）
        ))
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("hide".to_string(), "Hide")) // 隐藏应用窗口
        .add_item(CustomMenuItem::new("show".to_string(), "Show")) // 显示应用窗口
        .add_native_item(SystemTrayMenuItem::Separator) // 分割线
        .add_item(CustomMenuItem::new("quit".to_string(), "Quit")); // 退出

    // 设置在右键单击系统托盘时显示菜单
    SystemTray::new().with_menu(tray_menu)
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    // 获取应用窗口
    let window = app.get_window("main").unwrap();
    let parent_window = Some(&window);
    // 匹配点击事件
    match event {
        // 左键点击
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
        }
        // 右键点击
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        // 双击，macOS / Linux 不支持
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        // 根据菜单 id 进行事件匹配
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
            "edit_file" => {
                message(parent_window, "Eidt File", "TODO");
            }
            "new_file" => {
                message(parent_window, "New File", "TODO");
            }
            "quit" => {
                std::process::exit(0);
            }
            "show" => {
                window.show().unwrap();
            }
            "hide" => {
                window.hide().unwrap();
            }
            _ => {}
        },
        _ => {}
    }
}
