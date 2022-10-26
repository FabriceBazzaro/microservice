use bincode::{Decode, Encode};
use std::sync::{Arc, Mutex};
use std::fmt::Display;
use serde::Deserialize;

use crate::service::Service;
use crate::injectable;

pub use chrono;

#[repr(usize)]
#[derive(Encode, Decode, Clone, PartialEq, PartialOrd, Debug, Deserialize)]
#[serde(rename_all(serialize = "snake_case", deserialize="snake_case"))]
pub enum Level {
    Error = 1,
    Warn,
    Info,
    Debug,
    Trace
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Level
}


#[derive(Encode, Decode, PartialEq, Debug)]
pub struct LogMessage {
    pub timestamp: String,
    pub level: Level,
    pub emitter: String,
    pub message: String,
    pub file: String,
    pub line: u32
}

#[injectable(Service)]
pub trait Logger: Service + Send + Sync {
    fn enabled(&self, level: &Level) -> bool;
    fn log(&self, message: LogMessage);
}


use lazy_static::lazy_static;

lazy_static! {
    static ref INTERNAL_LOGGER: Mutex<Option<Arc<Mutex<dyn Logger>>>> = Mutex::new(None);
}

pub fn register_logger(logger: Arc<Mutex<dyn Logger>>) {
    let mut internal_logger = INTERNAL_LOGGER.lock().unwrap();
    *internal_logger = Some(logger);
}

pub fn log(level: &Level, message: String, emitter: String, file: String, line: u32) {
    let message = LogMessage {
        timestamp: chrono::Utc::now().to_rfc3339(),
        level: level.clone(),
        emitter,
        message,
        file,
        line
    };
    if let Some(internal_logger) = &*INTERNAL_LOGGER.lock().unwrap() {
        internal_logger.lock().unwrap().log(message);
    }
    else {
        println!("{} - {:?} - {} - {} - ({}:{})", message.timestamp, message.level, message.emitter, message.message, message.file, message.line);
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => (crate::logger::log(&crate::logger::Level::Error, std::format_args!($($arg)+).to_string(), env!("CARGO_PKG_NAME").to_string(), file!().to_string(), line!()))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => (crate::logger::log(&crate::logger::Level::Warn, std::format_args!($($arg)+).to_string(), env!("CARGO_PKG_NAME").to_string(), file!().to_string(), line!()))
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => (crate::logger::log(&crate::logger::Level::Info, std::format_args!($($arg)+).to_string(), env!("CARGO_PKG_NAME").to_string(), file!().to_string(), line!()))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => (crate::logger::log(&crate::logger::Level::Debug, std::format_args!($($arg)+).to_string(), env!("CARGO_PKG_NAME").to_string(), file!().to_string(), line!()))
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => (crate::logger::log(&crate::logger::Level::Trace, std::format_args!($($arg)+).to_string(), env!("CARGO_PKG_NAME").to_string(), file!().to_string(), line!()))
}
