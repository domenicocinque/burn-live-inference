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
        Self {
            model: ClassificationModel::new(),
        }
    }

    pub async fn run(self, mut rx: mpsc::UnboundedReceiver<PredictionRequest>) {
        while let Some(request) = rx.recv().await {
            let prediction = self.model.predict(request.image);
            let _ = request.response_tx.send(prediction);
        }
    }
}
