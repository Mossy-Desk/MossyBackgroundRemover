use std::path::PathBuf;

use tauri_plugin_dialog::DialogExt;

use crate::contracts::FileDialogTrait;

/// Native OS file dialogs via `tauri-plugin-dialog`. The only file that
/// imports `tauri_plugin_dialog` — everything above this reaches it
/// through `FileDialogTrait`.
pub struct TauriFileDialogTool {
    app_handle: tauri::AppHandle,
}

impl TauriFileDialogTool {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self { app_handle }
    }

    pub fn pick_image_file(&self) -> Result<Option<PathBuf>, String> {
        let picked = self
            .app_handle
            .dialog()
            .file()
            .add_filter("Images", &["png", "jpg", "jpeg"])
            .blocking_pick_file();

        match picked {
            None => Ok(None),
            Some(file_path) => file_path
                .into_path()
                .map(Some)
                .map_err(|e| format!("Failed to resolve picked file path: {}", e)),
        }
    }

    pub fn pick_png_save_path(&self, suggested_file_name: &str) -> Result<Option<PathBuf>, String> {
        let picked = self
            .app_handle
            .dialog()
            .file()
            .set_file_name(suggested_file_name)
            .add_filter("PNG Image", &["png"])
            .blocking_save_file();

        match picked {
            None => Ok(None),
            Some(file_path) => file_path
                .into_path()
                .map(Some)
                .map_err(|e| format!("Failed to resolve save file path: {}", e)),
        }
    }
}

impl FileDialogTrait for TauriFileDialogTool {
    fn pick_image_file(&self) -> Result<Option<PathBuf>, String> {
        TauriFileDialogTool::pick_image_file(self)
    }

    fn pick_png_save_path(&self, suggested_file_name: &str) -> Result<Option<PathBuf>, String> {
        TauriFileDialogTool::pick_png_save_path(self, suggested_file_name)
    }
}
