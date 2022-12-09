use std::env::current_exe;

use auto_launch::{AutoLaunch, AutoLaunchBuilder};
use tauri::State;

use crate::command::state::WebServerState;

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
