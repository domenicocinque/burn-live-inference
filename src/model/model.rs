use super::mnist::Model;

use burn::{
    backend::ndarray::NdArray,
    tensor::{Device, Tensor},
};

type Backend = NdArray<f32>;

/// Mean of the mnist dataset
pub const MNIST_MEAN: f32 = 0.1307;
/// Std of the mnist dataset
pub const MNIST_STD: f32 = 0.3081;

/// Wrapper for the classification model
pub struct ClassificationModel {
    model: Model<Backend>,
    device: Device<Backend>,
}

impl ClassificationModel {
    pub fn new() -> Self {
        let model: Model<Backend> = Model::default();
        let device = <Backend as burn::tensor::backend::Backend>::Device::default();
        ClassificationModel { model, device }
    }

    pub fn predict(&self, image: Vec<f32>) -> u8 {
        let mut input = Tensor::<Backend, 1>::from_floats(image.as_slice(), &self.device)
            .reshape([1, 1, 28, 28]);

        input = (input - MNIST_MEAN) / MNIST_STD;

        let output = self.model.forward(input);
        output.argmax(1).into_scalar() as u8
    }
}
