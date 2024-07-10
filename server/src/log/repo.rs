use crate::{database::storage::Storage, error::app_error::DynAppError};

use super::log::{Log, LogAttribute};

#[derive(Clone)]
pub struct LogRepo {
    storage: Storage,
}

impl LogRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }
    
    pub async fn save_log(&self, log: Log) -> Result<u64, DynAppError> {
        let cmd = String::from(
            "insert into 
                $1.log (log_type, log_message, log_timestamp, log_attributes)
            values
                ($2, $3, $4, $5, $6);
            "
        );

        self.storage.exec(cmd, &[&log.index, &log.message, &log.timestamp, &log.attributes]).await
    }

    fn parse_log_attributes(attributes : Option<Vec<LogAttribute>>) -> String {
        if let Some(attrs) = attributes {
            
        }

        String::from("ARRAY[]::json")
    }
}