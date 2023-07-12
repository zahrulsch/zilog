#![allow(dead_code)]
pub use anyhow;
pub use log;

use colored::Colorize;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

pub struct Zilog<'a> {
    app_name: &'a str,
    log_file_path: &'a str,
}

impl<'a> Zilog<'a> {
    pub fn builder() -> Zilog<'a> {
        Zilog {
            app_name: "zilog",
            log_file_path: "zilog.log",
        }
    }

    pub fn set_app_name(&mut self, name: &'a str) -> Self {
        Self {
            app_name: name,
            log_file_path: self.log_file_path,
        }
    }

    pub fn set_file_log_path(&mut self, path: &'a str) -> Self {
        Self {
            app_name: self.app_name,
            log_file_path: path,
        }
    }

    pub fn init(&self, max_level: log::LevelFilter) -> anyhow::Result<()> {
        let app_name_console = self.app_name.magenta().bold();
        let stdout_appender = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(&format!(
                "{} {{d(%d-%b-%Y %H:%M:%S)}} {{h({{l}})}} - {{m}}{{n}}",
                app_name_console
            ))))
            .build();

        let app_name_file = self.app_name;
        let file_appender = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new(&format!(
                "{} {{d(%d-%b-%Y %H:%M:%S)}} {{h({{l}})}} - {{m}}{{n}}",
                app_name_file
            ))))
            .build(self.log_file_path)?;

        let cfg = Config::builder()
            .appender(Appender::builder().build("console", Box::new(stdout_appender)))
            .appender(Appender::builder().build("file", Box::new(file_appender)))
            .build(
                Root::builder()
                    .appender("console")
                    .appender("file")
                    .build(max_level),
            )?;

        log4rs::init_config(cfg)?;
        log::set_max_level(max_level);

        Ok(())
    }
}
