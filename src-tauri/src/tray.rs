use tauri::{
    AppHandle, Manager, SystemTray, SystemTrayEvent,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    SystemTray::new()
}

// 菜单事件
pub fn handler(app: &AppHandle, event: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    match event {
        _ => {
            window.show().unwrap();
        }
    }
}
