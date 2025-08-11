mod mnist {
    include!(concat!(env!("OUT_DIR"), "/model/mnist.rs"));
}

pub mod model;
pub use model::ClassificationModel;
