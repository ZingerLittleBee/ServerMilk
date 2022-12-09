use tauri::{
    AppHandle, Manager, SystemTray, SystemTrayEvent,
};

// 托盘菜单
pub fn menu() -> SystemTray {
    SystemTray::new()
}

// 菜单事件
pub fn handler(app: &AppHandle, _: SystemTrayEvent) {
    let window = app.get_window("main").unwrap();
    {
        window.show().unwrap();
        window.set_always_on_top(true).unwrap();
    }
}
