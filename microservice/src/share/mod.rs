use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SimpleUrl {
    #[serde(skip_serializing_if = "Option::is_none")]
    protocol: Option<String>,
    host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<u16>
}

impl SimpleUrl {
    pub fn new(protocol: Option<String>, host: String, port: Option<u16>) -> Self {
        Self {
            protocol,
            host,
            port
        }
    }
}

impl Display for SimpleUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}{}", self.protocol.clone().unwrap_or("".into()), self.host, self.port.map_or("".to_string(), |v| format!(":{}", v)))
    }
}
