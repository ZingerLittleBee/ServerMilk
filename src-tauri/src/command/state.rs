use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use actix_rt::System;
use actix_web::dev::Server;
use tauri::State;

use crate::runner;

pub struct WebServerState(pub Mutex<Server>, pub Mutex<System>);

impl WebServerState {
    pub(crate) fn new(tup: (Server, System)) -> Self {
        WebServerState(Mutex::from(tup.0), Mutex::from(tup.1))
    }
}

#[tauri::command]
pub fn web_server_restart(app_handle: tauri::AppHandle, state: State<'_, WebServerState>) {
    println!("command web_server_restart");
    let _ = state.0.lock().unwrap().handle().stop(false);
    thread::sleep(Duration::from_millis(1000));
    let _ = state.1.lock().unwrap().stop();
    thread::sleep(Duration::from_millis(1000));
    let (srv, sys) = runner::web_runner(
        app_handle
            .path_resolver()
            .app_config_dir()
            .unwrap()
            .join("settings.json"),
        app_handle
            .path_resolver()
            .app_config_dir()
            .unwrap()
            .join("web.log"),
    );
    *state.0.lock().unwrap() = srv;
    *state.1.lock().unwrap() = sys;
}