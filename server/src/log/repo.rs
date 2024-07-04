use crate::{database::storage::Storage, error::app_error::DynAppError};

use super::log::Log;

#[derive(Clone)]
pub struct LogRepo {
    storage: Storage,
}

impl LogRepo {
    pub fn new(storage: Storage) -> Self {
        Self { storage: storage }
    }
    
    pub async fn save_log(&self, log: Log) -> Result<u64, DynAppError> {
        let cmd = String::from("");

        self.storage.exec(cmd, &[]).await
    }
}