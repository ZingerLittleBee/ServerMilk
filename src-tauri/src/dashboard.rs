use tauri::LogicalSize;
use window_shadows::set_shadow;
use crate::ext::window::WindowExt;
use crate::hacker;

#[tauri::command]
pub fn open_dashboard(handle: tauri::AppHandle) {
    let dashboard_window = tauri::WindowBuilder::new(
        &handle,
        "dashboard",
        tauri::WindowUrl::External(format!("http://localhost:{}", 9527).parse().unwrap())
    ).build().unwrap();

    dashboard_window.set_title("ServerMilk").unwrap();
    dashboard_window.set_decorations(true).unwrap();
    // dashboard_window.set_resizable(true).unwrap();

    dashboard_window.set_size(
        LogicalSize {
            width: 1400.0,
            height: 890.0,
        }
    ).unwrap();

    dashboard_window.set_min_size(
        Some(LogicalSize {
            width: 450.0,
            height: 200.0,
        })
    ).unwrap();

    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&dashboard_window, true).unwrap();

    #[cfg(target_os = "macos")]
    dashboard_window.set_transparent_titlebar(true);

    dashboard_window.eval(hacker::CRATE_DRAG_REGION).unwrap();
    dashboard_window.eval(hacker::MODIFY_NAVBAR).unwrap();
}
