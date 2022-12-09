use auto_launch::{AutoLaunch};
use tauri::State;

#[tauri::command]
pub fn is_enable_auto_launch(_: tauri::AppHandle, state: State<'_, AutoLaunch>) -> bool {
    state.is_enabled().unwrap()
}

#[tauri::command]
pub fn enable_auto_launch(_: tauri::AppHandle, state: State<'_, AutoLaunch>) -> () {
    state.enable().unwrap();
}

#[tauri::command]
pub fn disable_auto_launch(_: tauri::AppHandle, state: State<'_, AutoLaunch>) -> () {
    state.disable().unwrap()
}
