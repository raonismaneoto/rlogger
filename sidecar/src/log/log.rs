use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Log {
    pub log_type: String,
    pub message: String,
    pub timestamp: String,
    pub attributes: Vec<LogAttribute>
}

#[derive(Deserialize, Serialize)]
pub struct LogAttribute {
    name: String,
    value: String
}