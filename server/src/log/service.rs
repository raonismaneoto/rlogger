use crate::error::app_error::DynAppError;

use super::{log::Log, repo::LogRepo};

#[derive(Clone)]
pub struct LogService {
    repo: LogRepo,
}

impl LogService {
    pub fn new(repo: LogRepo) -> Self {
        Self {
            repo: repo
        }
    }

    pub async fn save_log(&self, log: Log) -> Result<(), DynAppError> {
        self.repo.save_log(log).await?;
        Ok(())
    }
}