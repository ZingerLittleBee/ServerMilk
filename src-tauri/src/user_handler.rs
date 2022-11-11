use std::fs::{File, OpenOptions};
use std::path::PathBuf;

use anyhow::Result;
use serde_json::from_reader;
use tauri::{App, Manager, Wry};

use crate::config::WebConfig;
use crate::dto::user_settings::UserSettings;

pub struct UserHandler<'a> {
    app: &'a App<Wry>,
}

impl<'a> UserHandler<'a> {
    pub fn new(app: &'a mut App<Wry>) -> Self {
        Self { app }
    }

    pub fn handle_user_settings(&self) -> Result<()> {
        self.sync_settings()
    }

    fn get_config_path(&self) -> Result<PathBuf> {
        let config_dir = self
            .app
            .path_resolver()
            .app_dir()
            .unwrap()
            .join("settings.json");
        Ok(config_dir)
    }

    fn get_user_settings(&self) -> UserSettings {
        let mut settings: UserSettings = Default::default();
        if let Ok(config_path) = self.get_config_path() {
            if let Ok(file) = File::open(config_path) {
                if let Ok(s) = from_reader::<File, UserSettings>(file) {
                    if s.theme.is_some() {
                        settings.theme = s.theme
                    }
                    if s.port.is_some() {
                        settings.port = s.port;
                    }
                    if s.is_enable_auto_launch.is_some() {
                        settings.is_enable_auto_launch = s.is_enable_auto_launch;
                    }
                }
            }
        }
        settings
    }

    fn sync_settings(&self) -> Result<()> {
        let user_settings = self.get_user_settings();
        if let Some(theme) = user_settings.theme {}
        if let Some(port) = user_settings.port {
            let f = OpenOptions::new()
                .write(true)
                .create(true)
                .open("config.yml")
                .expect("Couldn't open config.yml file");
            // serde_yaml::to_writer(f, &WebConfig::new(port)).unwrap();
        }
        // if let Some(is_enable) = user_settings.is_enable_auto_launch {
        //     if is_enable != is_enable_auto_launch(self.app.app_handle()) {
        //         if is_enable {
        //             enable_auto_launch(self.app.app_handle());
        //         } else {
        //             disable_auto_launch(self.app.app_handle());
        //         }
        //     }
        // }
        Ok(())
    }
}
