use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePreviewDto {
    pub width: u32,
    pub height: u32,
    pub png_base64: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundRemovalResultDto {
    pub original: ImagePreviewDto,
    pub result: ImagePreviewDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResultDto {
    pub saved_path: Option<String>,
}
