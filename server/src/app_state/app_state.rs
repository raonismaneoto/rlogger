use crate::{database::storage::{self, Storage}, log::{self, repo::LogRepo, service::LogService}};

#[derive(Clone)]
pub struct AppState {
    pub log_service: LogService
}

impl AppState {
    pub fn new() -> Self {
        let storage = Storage::new(String::from(""), String::from(""), String::from(""), String::from(""));
        let log_repo = LogRepo::new(storage.clone());
        let log_service = LogService::new(log_repo.clone());

        Self {
            log_service: log_service.clone()
        }
    }
}
