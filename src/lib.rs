//! Colored logging output for flexi_logger
//!
//! This crate provides one, but nice functionality - colored logs.
//!
//! # Examples
//!
//! See examples/auto.rs

use atty;
use chrono::Local;
use colored::{ColoredString, Colorize};
use flexi_logger::{Level, Record};
use std::env;
use std::str::FromStr;

#[macro_use]
extern crate failure;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorChoice {
    /// Always emit colors.
    Always,

    /// Use colors on decent terminal output. Don't use on dumb terminal or
    /// redirected stderr.
    Auto,

    /// Never emit colors.
    Never,
}

impl ColorChoice {
    /// Returns true if we should attempt to write colored output.
    fn should_attempt_color(&self) -> bool {
        match *self {
            ColorChoice::Always => true,
            ColorChoice::Never => false,
            ColorChoice::Auto => {
                if atty::is(atty::Stream::Stderr) {
                    match env::var("TERM") {
                        Err(_) => false,
                        Ok(k) => k != "dumb",
                    }
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Invalid color choice value")]
pub struct InvalidColorChoice;

impl FromStr for ColorChoice {
    type Err=InvalidColorChoice;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorChoice::Auto),
            "never" => Ok(ColorChoice::Never),
            "always" => Ok(ColorChoice::Always),
            _ => Err(InvalidColorChoice),
        }
    }
}

pub struct FormatterBuilder {
    color: ColorChoice,
}

impl FormatterBuilder {
    pub fn new() -> Self {
        FormatterBuilder {
            color: ColorChoice::Auto,
        }
    }

    pub fn with_color(mut self, color: ColorChoice) -> Self {
        self.color = color;
        self
    }

    pub fn build(self) -> fn(&mut std::io::Write, &Record) -> Result<(), std::io::Error> {
        if self.color.should_attempt_color() {
            color_formatter
        } else {
            no_color_formatter
        }
    }
}
impl Default for FormatterBuilder {
    fn default() -> Self {
        FormatterBuilder::new()
    }
}

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

#[deprecated(since = "0.4.0", note = "use FormatterBuilder instead")]
pub fn formatter(w: &mut std::io::Write, record: &Record) -> Result<(), std::io::Error> {
    color_formatter(w, record)
}

fn color_formatter(w: &mut std::io::Write, record: &Record) -> Result<(), std::io::Error> {
    let level = record.level();

    write!(
        w,
        "[{}] {} [{}:{}] {}",
        color(&Local::now().format("%Y-%m-%d %H:%M:%S%.6f %:z"), level),
        color(&level, level),
        color(&record.file().unwrap_or_default(), level),
        color(&record.line().unwrap_or_default(), level),
        &record.args()
    )
}

fn no_color_formatter(w: &mut std::io::Write, record: &Record) -> Result<(), std::io::Error> {
    let level = record.level();

    write!(
        w,
        "[{}] {} [{}:{}] {}",
        &Local::now().format("%Y-%m-%d %H:%M:%S%.6f %:z"),
        &level,
        &record.file().unwrap_or_default(),
        &record.line().unwrap_or_default(),
        &record.args()
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
