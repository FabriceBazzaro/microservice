use serde::Deserialize;

pub use log::{LevelFilter, Log};
pub use log::{trace, info, debug, warn, error};

use crate::service::Service;
use crate::injectable;

#[derive(Debug, Deserialize)]
pub struct LoggerConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: LevelFilter
}

#[injectable(Service)]
pub trait Logger: Log + Service + Send + Sync {}
