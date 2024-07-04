use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::app_state::app_state::AppState;
use crate::error::app_error::DynAppError;
use crate::log::log::Log;

pub async fn save_log_handler(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<Box<Log>>
) -> Response {
    match app_state.log_service.save_log(*payload).await {
        Ok(_) => Json({}).into_response(),
        Err(err) => get_error_response(err)
    }
}

fn get_error_response(error: DynAppError) -> Response {
    print!("{}", error.status_code());
    match StatusCode::from_u16(error.status_code() as u16) {
        Ok(status_code) => (status_code, Json(error.in_short())).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(error.in_short())).into_response(),
    }
}