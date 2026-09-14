pub mod contracts;
pub mod orchestrators;
pub mod tools;
pub mod workshop;

use std::sync::Mutex;

use contracts::{BackgroundRemovalResultDto, ExportResultDto, ImagePreviewDto};
use orchestrators::BackgroundRemoverOrchestrator;
use tauri::{Manager, State};
use tools::{ImageCodecTool, OrtSegmentationTool, TauriFileDialogTool};
use workshop::{BackgroundRemovalWorker, DialogWorker, ImageConverterWorker};

// All three commands below are declared `async fn` on purpose, even though
// none of them `.await` anything internally: a non-async `#[tauri::command]`
// runs on the main UI thread in Tauri 2, but every one of these does
// blocking work — `pick_and_load_image`/`export_result` call a native file
// dialog, whose own blocking API must NOT be called from the main thread
// (showing the dialog itself needs that thread's event loop), and
// `remove_background` runs model inference. `async fn` makes Tauri dispatch
// the command via `async_runtime::spawn` instead, off the main thread,
// matching tauri-plugin-dialog's own documented usage.

#[tauri::command]
async fn pick_and_load_image(
    state: State<'_, Mutex<BackgroundRemoverOrchestrator>>,
) -> Result<Option<ImagePreviewDto>, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_pick_and_load_image()
}

#[tauri::command]
async fn remove_background(
    state: State<'_, Mutex<BackgroundRemoverOrchestrator>>,
) -> Result<BackgroundRemovalResultDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_remove_background()
}

#[tauri::command]
async fn export_result(
    state: State<'_, Mutex<BackgroundRemoverOrchestrator>>,
) -> Result<ExportResultDto, String> {
    state
        .lock()
        .map_err(|_| "Failed to acquire state lock".to_string())?
        .handle_export_result()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let app_handle = app.handle();

            let dialog_worker = DialogWorker::new(TauriFileDialogTool::new(app_handle.clone()));

            let image_converter_worker = ImageConverterWorker::new(ImageCodecTool::new());

            let background_removal_worker =
                BackgroundRemovalWorker::new(OrtSegmentationTool::new());

            let orchestrator = BackgroundRemoverOrchestrator::new(
                dialog_worker,
                image_converter_worker,
                background_removal_worker,
            );

            app.manage(Mutex::new(orchestrator));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            pick_and_load_image,
            remove_background,
            export_result
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
