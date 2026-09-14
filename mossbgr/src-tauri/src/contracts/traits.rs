//! Traits ("contracts") that decouple workers from concrete tools. Workers
//! depend on these, never on a `tools::*` type directly — this is what
//! keeps a worker unit-testable with a mock and isolates the domain from
//! third-party crate churn (swapping the HTTP client, the image codec, or
//! the ONNX runtime should only ever touch `tools/`).

use std::path::PathBuf;

use image::{DynamicImage, GrayImage};

/// Native OS file dialogs. `Ok(None)` means the user cancelled — that is a
/// normal outcome, not an error.
pub trait FileDialogTrait {
    fn pick_image_file(&self) -> Result<Option<PathBuf>, String>;
    fn pick_png_save_path(&self, suggested_file_name: &str) -> Result<Option<PathBuf>, String>;
}

/// Decoding/encoding image bytes.
pub trait ImageCodecTrait {
    fn load_from_path(&self, path: &std::path::Path) -> Result<DynamicImage, String>;
    fn encode_png_base64(&self, image: &DynamicImage) -> Result<String, String>;
    fn save_png_to_path(&self, image: &DynamicImage, path: &std::path::Path) -> Result<(), String>;
}

/// Running the segmentation model. Returns a single-channel mask
/// (0 = background, 255 = foreground) at the model's native output
/// resolution — NOT necessarily the input image's resolution. Upscaling
/// the mask back to the original size is the caller's (a worker's) job,
/// not this trait's.
pub trait SegmentationModelTrait {
    fn predict_mask(&self, image: &DynamicImage) -> Result<GrayImage, String>;
}
