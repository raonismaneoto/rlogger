use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub log_type: String,
    pub message: String,
    pub timestamp: String,
    pub attributes: Vec<LogAttribute>,
    pub index: String,
}

#[derive(Deserialize, Serialize)]
pub struct LogAttribute {
    name: String,
    value: String,
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Message: {}, Type: {}, Timestamp: {}, Index: {}, AttrsAmount: {}",
            self.message,
            self.log_type,
            self.timestamp,
            self.index,
            self.attributes.len()
        )
    }
}
