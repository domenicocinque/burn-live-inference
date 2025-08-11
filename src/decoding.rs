use base64::prelude::*;
use image::{DynamicImage, ImageReader};

pub const MNIST_IMAGE_SIZE: usize = 784; // 28x28

fn image_to_vec(img: &DynamicImage) -> Vec<f32> {
    let gray_img = img.to_luma8();
    let raw = gray_img.into_raw();

    let result: Vec<f32> = raw.into_iter().map(|v| v as f32 / 255.0).collect();

    assert_eq!(result.len(), MNIST_IMAGE_SIZE);
    result
}

/// Takes an image in b64 and decodes it to a flat vector
///
/// It returns an error if the image is not 28x28.
pub fn decode_and_process_image(image_b64: &str) -> Result<Vec<f32>, String> {
    let bytes = BASE64_STANDARD
        .decode(image_b64)
        .map_err(|_| "Error while decoding base64".to_string())?;

    let img = ImageReader::new(std::io::Cursor::new(bytes))
        .with_guessed_format()
        .map_err(|_| "Invalid image format".to_string())?
        .decode()
        .map_err(|_| "Failed to decode image".to_string())?;

    if img.width() != 28 || img.height() != 28 {
        return Err(format!(
            "Image must be 28x28 pixels, got {}x{}",
            img.width(),
            img.height()
        ));
    }

    Ok(image_to_vec(&img))
}
