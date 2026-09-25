use std::path::PathBuf;

use image::{DynamicImage, GrayImage};

pub trait FileDialogTrait {
    fn pick_image_file(&self) -> Result<Option<PathBuf>, String>;
    fn pick_png_save_path(&self, suggested_file_name: &str) -> Result<Option<PathBuf>, String>;
}

pub trait ImageCodecTrait {
    fn load_from_path(&self, path: &std::path::Path) -> Result<DynamicImage, String>;
    fn encode_png_base64(&self, image: &DynamicImage) -> Result<String, String>;
    fn save_png_to_path(&self, image: &DynamicImage, path: &std::path::Path) -> Result<(), String>;
}

pub trait SegmentationModelTrait {
    fn predict_mask(&self, image: &DynamicImage) -> Result<GrayImage, String>;
}
