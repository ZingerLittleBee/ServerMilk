use anyhow::Result;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct WebConfig {
    server: Port,
}

impl Default for WebConfig {
    fn default() -> Self {
        WebConfig {
            server: Port { port: 8081 },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Port {
    port: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {}

impl Config {
    pub fn init_logging() {
        // init logging
        let stdout: ConsoleAppender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)} {T} {l}] {m}{n}",
            )))
            .build();

        // Logging to log file.
        let logfile = FileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)} {T} {l}] {m}{n}",
            )))
            .build(Config::log_path())
            .unwrap();

        let log_config = log4rs::config::Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(
                Root::builder()
                    .appender("stdout")
                    .appender("logfile")
                    .build(LevelFilter::Info),
            )
            .unwrap();

        log4rs::init_config(log_config).unwrap();
    }

    fn get_config_yml() -> Result<WebConfig> {
        let mut config_path = PathBuf::from("config.yml");
        if let Ok(current_exe) = env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                config_path = parent.join("config.yml");
            }
        }

        let config_file = File::open(config_path)?;
        Ok(serde_yaml::from_reader::<File, WebConfig>(config_file)?)
    }

    pub fn get_server_port() -> u16 {
        let d = Config::get_config_yml().unwrap_or_default();
        d.server.port
    }

    pub fn current_dir() -> PathBuf {
        if let Ok(current_exe) = env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                return parent.to_path_buf();
            }
        }
        env::current_dir().expect("获取当前目录失败, 权限不足或当前目录不存在")
    }

    pub fn log_path() -> PathBuf {
        Config::current_dir().join("web.log")
    }
}
