use std::collections::HashMap;
use std::sync::Mutex;

use image::imageops::FilterType;
use image::{DynamicImage, GrayImage, Luma};
use ndarray::{Array4, ArrayViewD};
use ort::session::{Session, SessionInputValue};
use ort::value::TensorRef;

use crate::contracts::SegmentationModelTrait;

// This is a third-party model we did not train: U²-Net (Qin et al.,
// "U²-Net: Going Deeper with Nested U-Structure for Salient Object
// Detection", Pattern Recognition, 2020 — https://github.com/xuebinqin/U-2-Net,
// Apache License 2.0), packaged as ONNX by danielgatis/rembg (MIT License).
// Full attribution + license text: ../../models/NOTICE.md and
// ../../models/LICENSE-u2net.
const MODEL_BYTES: &[u8] = include_bytes!("../../models/u2netp.onnx");

const MODEL_INPUT_SIZE: u32 = 320;
const NORM_MEAN: [f32; 3] = [0.485, 0.456, 0.406];
const NORM_STD: [f32; 3] = [0.229, 0.224, 0.225];

pub struct OrtSegmentationTool {
    session: Mutex<Option<Session>>,
}

impl OrtSegmentationTool {
    pub fn new() -> Self {
        Self {
            session: Mutex::new(None),
        }
    }

    pub fn predict_mask(&self, image: &DynamicImage) -> Result<GrayImage, String> {
        let mut guard = self
            .session
            .lock()
            .map_err(|_| "Segmentation session lock was poisoned".to_string())?;

        if guard.is_none() {
            let session = Session::builder()
                .map_err(|e| format!("Failed to create ONNX Runtime session builder: {}", e))?
                .commit_from_memory(MODEL_BYTES)
                .map_err(|e| format!("Failed to load segmentation model: {}", e))?;
            *guard = Some(session);
        }
        let session = guard.as_mut().expect("session was just initialized");

        let input_name = session.inputs()[0].name().to_string();
        let output_name = session.outputs()[0].name().to_string();

        let input_tensor = preprocess(image);

        let mut inputs: HashMap<String, SessionInputValue> = HashMap::new();
        inputs.insert(
            input_name,
            TensorRef::from_array_view(input_tensor.view())
                .map_err(|e| format!("Failed to build input tensor: {}", e))?
                .into(),
        );

        let outputs = session
            .run(inputs)
            .map_err(|e| format!("Segmentation inference failed: {}", e))?;

        let output_value = outputs
            .get(output_name.as_str())
            .ok_or_else(|| format!("Model output `{}` not found", output_name))?;

        let output_array = output_value
            .try_extract_array::<f32>()
            .map_err(|e| format!("Failed to extract segmentation output: {}", e))?;

        postprocess_to_mask(output_array)
    }
}

impl Default for OrtSegmentationTool {
    fn default() -> Self {
        Self::new()
    }
}

fn preprocess(image: &DynamicImage) -> Array4<f32> {
    let resized = image
        .resize_exact(MODEL_INPUT_SIZE, MODEL_INPUT_SIZE, FilterType::Lanczos3)
        .to_rgb8();

    let max_value = resized
        .pixels()
        .flat_map(|pixel| pixel.0)
        .map(f32::from)
        .fold(1e-6_f32, f32::max);

    let size = MODEL_INPUT_SIZE as usize;
    let mut input = Array4::<f32>::zeros((1, 3, size, size));
    for (x, y, pixel) in resized.enumerate_pixels() {
        for channel in 0..3 {
            let value = pixel[channel] as f32 / max_value;
            input[[0, channel, y as usize, x as usize]] =
                (value - NORM_MEAN[channel]) / NORM_STD[channel];
        }
    }
    input
}

fn postprocess_to_mask(output: ArrayViewD<f32>) -> Result<GrayImage, String> {
    let shape = output.shape();
    let [_, _, height, width] = shape else {
        return Err(format!(
            "Unexpected segmentation output rank {} (shape {:?}), expected (N, C, H, W)",
            shape.len(),
            shape
        ));
    };
    let (height, width) = (*height, *width);

    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    for y in 0..height {
        for x in 0..width {
            let value = output[[0, 0, y, x]];
            min = min.min(value);
            max = max.max(value);
        }
    }
    let range = (max - min).max(1e-6);

    let mut mask = GrayImage::new(width as u32, height as u32);
    for y in 0..height {
        for x in 0..width {
            let value = output[[0, 0, y, x]];
            let normalized = ((value - min) / range).clamp(0.0, 1.0);
            mask.put_pixel(x as u32, y as u32, Luma([(normalized * 255.0) as u8]));
        }
    }
    Ok(mask)
}

impl SegmentationModelTrait for OrtSegmentationTool {
    fn predict_mask(&self, image: &DynamicImage) -> Result<GrayImage, String> {
        OrtSegmentationTool::predict_mask(self, image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;
    use md5::{Digest, Md5};
    use ndarray::Array;

    const MODEL_MD5: &str = "8e83ca70e441ab06c318d82300c84806";

    #[test]
    fn embedded_model_bytes_match_published_checksum() {
        let mut hasher = Md5::new();
        hasher.update(MODEL_BYTES);
        let digest: String = hasher
            .finalize()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect();

        assert_eq!(digest, MODEL_MD5);
    }

    #[test]
    fn preprocess_produces_normalized_shape() {
        let image =
            DynamicImage::ImageRgba8(RgbaImage::from_pixel(64, 32, image::Rgba([255, 0, 0, 255])));

        let tensor = preprocess(&image);

        assert_eq!(tensor.shape(), &[1, 3, 320, 320]);
        let expected_r = (1.0 - NORM_MEAN[0]) / NORM_STD[0];
        assert!((tensor[[0, 0, 0, 0]] - expected_r).abs() < 1e-4);
    }

    #[test]
    fn postprocess_to_mask_stretches_full_range() {
        // A 1x1x2x2 output with two distinct values should stretch to 0/255.
        let raw = Array::from_shape_vec((1, 1, 2, 2), vec![0.2_f32, 0.2, 0.2, 0.8]).unwrap();

        let mask = postprocess_to_mask(raw.into_dyn().view()).unwrap();

        assert_eq!(mask.dimensions(), (2, 2));
        assert_eq!(mask.get_pixel(0, 0).0[0], 0);
        assert_eq!(mask.get_pixel(1, 1).0[0], 255);
    }

    #[test]
    fn predict_mask_runs_the_real_embedded_model_end_to_end() {
        let tool = OrtSegmentationTool::new();
        let image = DynamicImage::ImageRgba8(RgbaImage::from_pixel(
            50,
            30,
            image::Rgba([120, 200, 80, 255]),
        ));

        let mask = tool.predict_mask(&image).unwrap();

        assert_eq!(mask.dimensions(), (MODEL_INPUT_SIZE, MODEL_INPUT_SIZE));
    }
}
