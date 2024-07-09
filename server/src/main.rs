use app_state::app_state::AppState;
use axum::{
    routing::{get, patch, post},
    Error, Router,
};
use handlers::logs::save_log_handler;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

pub mod log;
pub mod app_state;
pub mod handlers;
pub mod error;
pub mod database;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    start_web_server().await;
}

async fn start_web_server() -> Result<(), Error> {
    let app_state = Arc::new(AppState::new());

    let app = Router::new()
        .route(
            "/api/rlogger/health-check",
            get(|| async { "RLogger server is online" }),
        )
        .route("/api/rlogger/logs", post(save_log_handler))
        // .route_layer(map_request_with_state(app_state.clone(), auth_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([192, 168, 0, 7], 5000));
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("listening on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();

    Ok(())
}