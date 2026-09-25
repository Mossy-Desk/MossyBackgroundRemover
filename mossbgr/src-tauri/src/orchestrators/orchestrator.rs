use std::path::{Path, PathBuf};

use image::DynamicImage;

use crate::contracts::*;
use crate::tools::{ImageCodecTool, OrtSegmentationTool, TauriFileDialogTool};
use crate::workshop::{BackgroundRemovalWorker, DialogWorker, ImageConverterWorker};

const EXPORT_DEFAULT_FILE_NAME: &str = "background-removed.png";

pub struct BackgroundRemoverOrchestrator {
    dialog_worker: DialogWorker<TauriFileDialogTool>,
    image_converter_worker: ImageConverterWorker<ImageCodecTool>,
    background_removal_worker: BackgroundRemovalWorker<OrtSegmentationTool>,
}

impl BackgroundRemoverOrchestrator {
    pub fn new(
        dialog_worker: DialogWorker<TauriFileDialogTool>,
        image_converter_worker: ImageConverterWorker<ImageCodecTool>,
        background_removal_worker: BackgroundRemovalWorker<OrtSegmentationTool>,
    ) -> Self {
        Self {
            dialog_worker,
            image_converter_worker,
            background_removal_worker,
        }
    }

    pub fn get_path_from_dialog(&mut self) -> Result<Option<ImagePreviewDto>, String> {
        let Some(path) = self.dialog_worker.pick_image_file()? else {
            return Ok(None);
        };

        self.load_image_from_path(&path).map(Some)
    }

    pub fn get_path_from_drop(&mut self, path: PathBuf) -> Result<ImagePreviewDto, String> {
        self.load_image_from_path(&path)
    }

    fn load_image_from_path(&mut self, path: &Path) -> Result<ImagePreviewDto, String> {
        let image = self.image_converter_worker.decode_from_path(path)?;
        let preview = self.image_converter_worker.to_preview_dto(&image)?;
        self.background_removal_worker.set_current_image(image);

        Ok(preview)
    }

    pub fn handle_remove_background(&mut self) -> Result<BackgroundRemovalResultDto, String> {
        let original = self
            .background_removal_worker
            .current_image()
            .cloned()
            .ok_or_else(|| "No image loaded — pick an image first".to_string())?;

        let result = self.background_removal_worker.remove_background()?;

        Ok(BackgroundRemovalResultDto {
            original: self.image_converter_worker.to_preview_dto(&original)?,
            result: self
                .image_converter_worker
                .to_preview_dto(&DynamicImage::ImageRgba8(result))?,
        })
    }

    pub fn handle_export_result(&self) -> Result<ExportResultDto, String> {
        let result = self
            .background_removal_worker
            .last_result()
            .ok_or_else(|| "No background-removal result to export yet".to_string())?;

        let Some(path) = self
            .dialog_worker
            .pick_save_path(EXPORT_DEFAULT_FILE_NAME)?
        else {
            return Ok(ExportResultDto { saved_path: None });
        };

        self.image_converter_worker
            .encode_to_path(&DynamicImage::ImageRgba8(result.clone()), &path)?;

        Ok(ExportResultDto {
            saved_path: Some(path.display().to_string()),
        })
    }
}
