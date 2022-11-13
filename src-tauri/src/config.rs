use std::env;
use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WebConfig {
    server: Port,
}

impl WebConfig {
    pub fn new(port: u16) -> Self {
        Self {
            server: Port { port },
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Port {
    port: u16,
}

impl Default for Port {
    fn default() -> Self {
        Port { port: 9527 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Config {}

impl Config {
    pub fn init_logging(log_path: PathBuf) {
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
            .build(log_path)
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

        match log4rs::init_config(log_config) {
            Ok(_) => {}
            Err(_) => println!("Failed to initialize logging"),
        }
    }

    // fn get_config_yml() -> Result<WebConfig> {
    //     let config_file = File::open("config.yml")?;
    //     Ok(serde_yaml::from_reader::<File, WebConfig>(config_file)?)
    // }

    fn read_config(path: PathBuf) -> Result<Port> {
        let config_file = File::open(path)?;
        Ok(serde_json::from_reader::<File, Port>(config_file)?)
    }

    pub fn get_server_port(path: PathBuf) -> u16 {
        let d = Self::read_config(path).unwrap_or_default();
        d.port
    }

    pub fn current_dir() -> PathBuf {
        if let Ok(current_exe) = env::current_exe() {
            if let Some(parent) = current_exe.parent() {
                return parent.to_path_buf();
            }
        }
        env::current_dir().expect("获取当前目录失败, 权限不足或当前目录不存在")
    }
}
