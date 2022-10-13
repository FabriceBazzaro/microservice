use std::sync::{Arc, Mutex};
use std::ffi::OsString;
use serde::Deserialize;
pub use serde_yaml::Value;

use crate::error::*;
use crate::service::Service;
use crate::injectable;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Config - File not found: {file}")]
    FileNotFound { file: String },

    #[error("Config - Parsing error for the file {file}")]
    ParseError { file: String },

    #[error("Config - Deserialization error for the field {field} and the value: {value:?}")]
    DeserializationError { source: serde_yaml::Error, field: String, value: Value },

    #[error("Config - Environment variable {field} not found \n {src}")]
    EnvironmentVariableError { src: String, field: String },

    #[error("Config - Field not found: {field:?}")]
    FieldNotFound { field: String },

    #[error("Config - Unable to convert hostname in string: {hostname:?}")]
    HostnameConversionFailed { hostname: OsString }

}


#[injectable(Service)]
pub trait Config: Service {
    fn get(&self, field: &str) -> Result<Value>;
}

pub fn get_from_config<'de, T>(conf: Arc<Mutex<dyn Config>>, field: &str) -> Result<T> where T: Deserialize<'de> + Sized {
    let value: Value = conf.lock().unwrap().get(field)?;
    T::deserialize(value.clone()).map_err(|source| (ConfigError::DeserializationError { source, field: field.to_string(), value: value.clone() }).into())
}

pub fn get_or_from_config<'de, T>(conf: Arc<Mutex<dyn Config>>, field: &str, default: T) -> T where T: Deserialize<'de> + Sized {
    if let Ok(value) = conf.lock().unwrap().get(field) {
        T::deserialize(value.clone()).unwrap_or(default)
    }
    else {
        default
    }
}
