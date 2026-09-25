use std::io::Cursor;
use std::path::Path;

use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use image::{DynamicImage, ImageFormat};

use crate::contracts::ImageCodecTrait;

pub struct ImageCodecTool;

impl ImageCodecTool {
    pub fn new() -> Self {
        Self
    }

    pub fn load_from_path(&self, path: &Path) -> Result<DynamicImage, String> {
        image::open(path).map_err(|e| format!("Failed to decode image {}: {}", path.display(), e))
    }

    pub fn encode_png_base64(&self, image: &DynamicImage) -> Result<String, String> {
        let mut bytes: Vec<u8> = Vec::new();
        image
            .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .map_err(|e| format!("Failed to encode PNG: {}", e))?;
        Ok(BASE64.encode(bytes))
    }

    pub fn save_png_to_path(&self, image: &DynamicImage, path: &Path) -> Result<(), String> {
        image
            .save_with_format(path, ImageFormat::Png)
            .map_err(|e| format!("Failed to write PNG to {}: {}", path.display(), e))
    }
}

impl Default for ImageCodecTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageCodecTrait for ImageCodecTool {
    fn load_from_path(&self, path: &Path) -> Result<DynamicImage, String> {
        ImageCodecTool::load_from_path(self, path)
    }

    fn encode_png_base64(&self, image: &DynamicImage) -> Result<String, String> {
        ImageCodecTool::encode_png_base64(self, image)
    }

    fn save_png_to_path(&self, image: &DynamicImage, path: &Path) -> Result<(), String> {
        ImageCodecTool::save_png_to_path(self, image, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    fn sample_image() -> DynamicImage {
        DynamicImage::ImageRgba8(RgbaImage::from_pixel(4, 4, Rgba([10, 20, 30, 255])))
    }

    #[test]
    fn encode_png_base64_round_trips_through_decode() {
        let tool = ImageCodecTool::new();
        let original = sample_image();

        let encoded = tool.encode_png_base64(&original).unwrap();
        let bytes = BASE64.decode(encoded).unwrap();
        let decoded = image::load_from_memory(&bytes).unwrap();

        assert_eq!(decoded.width(), original.width());
        assert_eq!(decoded.height(), original.height());
    }

    #[test]
    fn save_png_to_path_then_load_from_path_round_trips() {
        let tool = ImageCodecTool::new();
        let original = sample_image();
        let dir = std::env::temp_dir().join(format!("mossbgr-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("sample.png");

        tool.save_png_to_path(&original, &path).unwrap();
        let loaded = tool.load_from_path(&path).unwrap();

        assert_eq!(loaded.width(), original.width());
        assert_eq!(loaded.height(), original.height());

        std::fs::remove_dir_all(&dir).ok();
    }
}
