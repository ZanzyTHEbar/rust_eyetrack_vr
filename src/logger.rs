//! Crate Logger
pub use crate::utils::colors::Color;
use env_logger::filter::{Builder, Filter};
use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use std::format as f;

pub struct Logger {
    inner: Filter,
}

impl Logger {
    pub fn new(log_level: LevelFilter) -> Logger {
        let mut builder = Builder::new();

        //builder
        //    .filter(None, LevelFilter::Info)
        //    .filter(Some("desktop_cleaner"), LevelFilter::Debug);

        builder.filter_level(log_level);
        Logger {
            inner: builder.build(),
        }
    }

    pub fn init(log_level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Self::new(log_level);
        log::set_max_level(logger.inner.filter());
        log::set_boxed_logger(Box::new(logger))
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.inner.matches(record) {
            println!(
                "{}",
                format_args!(
                    "{} {}",
                    Color::new(
                        f!(
                            "[EyeTrackVR - {}]:",
                            Color::new(record.level().as_str())
                                .map_level(record.level())
                                .bold(),
                        )
                        .as_str()
                    )
                    .bold()
                    .green(),
                    Color::new(f!("{}", record.args()).as_str()).cyan()
                )
            );
        }
    }

    fn flush(&self) {}
}

#[macro_export]
macro_rules! dc_stderr {
    ($($arg:tt)+) => (eprintln!("{}", f!("{} {}", Color::new("[EyeTrackVR]:").bold().green(), Color::new($($arg)+).red())));
}

#[macro_export]
macro_rules! dc_stdout {
    ($($arg:tt)+) => (println!("{}", f!("{} {}", Color::new("[EyeTrackVR]:").bold().green(), Color::new($($arg)+).green())));
}

pub(crate) use dc_stderr;
pub(crate) use dc_stdout;
