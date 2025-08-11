mod decoding;
mod handlers;
mod model;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{health, predict};
use model::ClassificationModel;
use std::sync::Arc;
use tokio::sync::Mutex;

/// The app entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let model = Arc::new(Mutex::new(ClassificationModel::new()));

    let app = Router::new()
        .route("/status", get(health))
        .route("/predict", post(predict))
        .with_state(model);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    println!("Server running on http://0.0.0.0:8000");

    axum::serve(listener, app).await?;
    Ok(())
}
