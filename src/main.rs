mod decoding;
mod handlers;
mod model;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{health, predict};
use tokio::sync::mpsc;

use model::worker::ModelWorker;

/// The app entrypoint
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (sender, receiver) = mpsc::unbounded_channel();

    let worker_handle = tokio::spawn(async move {
        let worker = ModelWorker::new();
        worker.run(receiver).await
    });

    let app = Router::new()
        .route("/status", get(health))
        .route("/predict", post(predict))
        .with_state(sender);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    println!("Server running on http://0.0.0.0:8000");

    tokio::select! {
        _ = axum::serve(listener, app) => {},
        _ = worker_handle => {},
    }

    Ok(())
}
