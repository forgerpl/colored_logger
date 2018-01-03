//! Colored logging output for flexi_logger
//!
//! This crate provides one, but nice functionality - colored logs.
//!
//! # Examples
//!
//! ```no_run
//! #[macro_use]
//! extern crate log;
//! extern crate flexi_logger;
//! extern crate colored_logger;
//!
//! use colored_logger::formatter;
//!
//! fn main() {
//!    flexi_logger::LogOptions::new()
//!        .format(formatter)
//!        .init(Some("info".to_string()))
//!        .unwrap();
//!
//!
//!    info!("This is a test message");
//!    error!("Error!");
//! }
//! ```

extern crate flexi_logger;
extern crate colored;
extern crate chrono;

use flexi_logger::{Record, Level};
use colored::{Colorize, ColoredString};
use chrono::Local;


pub fn formatter(record: &Record) -> String {
    let level = record.level();

    fn color<T: ToString>(fstr: &T, level: Level) -> ColoredString {
        use self::Level::*;
        let fstr = fstr.to_string();
        let fstr = fstr.as_str();

        match level {
            Error => fstr.red(),
            Warn => fstr.yellow(),
            Info => fstr.green(),
            Debug => fstr.blue(),
            Trace => fstr.magenta(),
        }
    }

    format!("[{}] {} [{}:{}] {}",
            color(&Local::now().format("%Y-%m-%d %H:%M:%S%.6f %:z"), level),
            color(&level, level),
            color(&record.file().unwrap_or_default(), level),
            color(&record.line().unwrap_or_default(), level),
            &record.args())

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
