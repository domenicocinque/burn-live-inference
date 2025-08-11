use super::model::ClassificationModel;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

/// Request to the model
pub struct PredictionRequest {
    pub image: Vec<f32>,
    pub response_tx: oneshot::Sender<u8>,
}

/// The model worker handles the requests to the model in a
/// background task
pub struct ModelWorker {
    model: ClassificationModel,
}

impl ModelWorker {
    pub fn new() -> Self {
        tracing::info!("Initializing model worker");
        Self {
            model: ClassificationModel::new(),
        }
    }

    #[tracing::instrument(skip(self, rx))]
    pub async fn run(self, mut rx: mpsc::UnboundedReceiver<PredictionRequest>) {
        tracing::info!("Model worker started");

        while let Some(request) = rx.recv().await {
            tracing::debug!("Received prediction request");
            let prediction = self.model.predict(request.image);

            if request.response_tx.send(prediction).is_err() {
                tracing::warn!("Failed to send prediction response - receiver dropped");
            }
        }

        tracing::info!("Model worker shutting down");
    }
}
