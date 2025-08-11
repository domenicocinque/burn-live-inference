use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::decoding::decode_and_process_image;
use crate::model::ClassificationModel;

#[derive(Deserialize)]
pub struct ImagePayload {
    pub image_b64: String,
}

/// A simple healthcheck
pub async fn health() -> &'static str {
    "Ok"
}

/// Predict the label for an input image
pub async fn predict(
    State(model): State<Arc<Mutex<ClassificationModel>>>,
    Json(payload): Json<ImagePayload>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let image_vec = match decode_and_process_image(&payload.image_b64) {
        Ok(vec) => vec,
        Err(e) => return Err((StatusCode::BAD_REQUEST, e)),
    };

    let prediction = model.lock().await.predict(image_vec);

    Ok((StatusCode::OK, prediction.to_string()))
}
