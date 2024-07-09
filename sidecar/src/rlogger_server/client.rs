use std::error::Error;

use reqwest::StatusCode;

use crate::{
    error::{app_error::DynAppError, default::DefaultAppError},
    log::log::Log,
};

pub struct RloggerServerClient;

impl RloggerServerClient {
    pub async fn SaveLog(log: Log) -> Result<(), DynAppError> {
        let client = reqwest::Client::new();
        let maybe_resp = client.post("localhost:8080/api/rlogger")
            .json(&log)
            .send()
            .await;

        match maybe_resp {
            Ok(resp) => {
                if !resp.status().is_success() {
                    let status_code = resp.status().clone();

                    let maybe_msg = resp.text().await;
                    let msg;

                    match maybe_msg {
                        Ok(err_msg) => {
                            msg = err_msg;
                        }
                        Err(err) => {
                            msg = String::from("Error while saving the log");
                        }
                    }
                    return Err(Box::new(DefaultAppError {
                        message: Some(msg),
                        status_code: status_code.as_u16() as i32,
                    }));
                }
            }
            Err(err) => {
                return Err(Box::new(DefaultAppError {
                    message: Some(err.to_string()),
                    status_code: err
                        .status()
                        .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
                        .as_u16() as i32,
                }))
            }
        }

        Ok(())
    }
}
