//! DTOs crossing the Tauri boundary (commands + events). These are distinct
//! from the internal domain models the workshop layer operates on — a
//! worker converts explicitly between the two rather than using a DTO as
//! its own working type.

use serde::{Deserialize, Serialize};

/// A preview image sent to the frontend: PNG bytes, base64-encoded, plus
/// its pixel dimensions. Preview images may be downscaled from the
/// worker's full-resolution domain image to keep the IPC payload sane —
/// export always re-encodes the full-resolution image, never a preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImagePreviewDto {
    pub width: u32,
    pub height: u32,
    pub png_base64: String,
}

/// Result of `remove_background`: both the (possibly downscaled) original
/// and the result preview, so the frontend can render a before/after view
/// from a single response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundRemovalResultDto {
    pub original: ImagePreviewDto,
    pub result: ImagePreviewDto,
}

/// Result of `export_result`. `saved_path` is `None` when the user
/// cancelled the save dialog — a normal outcome, not an error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResultDto {
    pub saved_path: Option<String>,
}
