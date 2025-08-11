use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

use crate::decoding::decode_and_process_image;
use crate::error::ApiError;
use crate::model::worker::PredictionRequest;

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
    State(sender): State<mpsc::UnboundedSender<PredictionRequest>>,
    Json(payload): Json<ImagePayload>,
) -> Result<(StatusCode, String), ApiError> {
    let image_vec = match decode_and_process_image(&payload.image_b64) {
        Ok(vec) => vec,
        Err(e) => return Err(ApiError::BadRequest(format!("Image decoding error: {}", e))),
    };

    let (response_sender, response_receiver) = oneshot::channel();

    let prediction_request = PredictionRequest {
        image: image_vec,
        response_tx: response_sender,
    };

    let _ = sender.send(prediction_request);

    match response_receiver.await {
        Ok(prediction) => Ok((StatusCode::OK, prediction.to_string())),
        Err(e) => Err(ApiError::InternalServer(e.to_string())),
    }
}
