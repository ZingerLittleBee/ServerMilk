use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;
use std::path::Path;

pub fn init_log(log_dir: &Path) {
    let file = Box::new(
        FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)} {T} {h({l})}] {m}{n}",
            )))
            .build(log_dir.join("web.log"))
            .unwrap(),
    );

    let console = Box::new(
        ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(
                "[{d(%Y-%m-%d %H:%M:%S)} {T} {h({l})}] {m}{n}",
            )))
            .build(),
    );

    // The `Config` builder allows configuring the `max_log_level` manually:
    let config = Config::builder()
        .appender(Appender::builder().build("file", file))
        .appender(Appender::builder().build("console", console))
        .logger(
            Logger::builder()
                .appender("file")
                .appender("console")
                .additive(false)
                .build("app.servermilk", LevelFilter::Info),
        )

        .build(Root::builder().appender("file")
            .appender("console")
            .build(LevelFilter::Debug))
        .unwrap();

    log4rs::init_config(config).unwrap();
}
