use auto_launch::AutoLaunchBuilder;
use std::env::current_exe;

#[tauri::command]
pub fn is_enable_auto_launch(app_handle: tauri::AppHandle) -> bool {
    let app_name = &app_handle.package_info().name;
    let current_exe = current_exe().unwrap();
    let auto_start = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    auto_start.is_enabled().unwrap()
}

#[tauri::command]
pub fn enable_auto_launch(app_handle: tauri::AppHandle) -> () {
    let app_name = &app_handle.package_info().name;
    let current_exe = current_exe().unwrap();
    let auto_start = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();
    auto_start.enable().unwrap()
}

#[tauri::command]
pub fn disable_auto_launch(app_handle: tauri::AppHandle) -> () {
    let app_name = &app_handle.package_info().name;
    let current_exe = current_exe().unwrap();
    let auto_start = AutoLaunchBuilder::new()
        .set_app_name(&app_name)
        .set_app_path(&current_exe.to_str().unwrap())
        .set_use_launch_agent(true)
        .build()
        .unwrap();

    auto_start.disable().unwrap()
}
