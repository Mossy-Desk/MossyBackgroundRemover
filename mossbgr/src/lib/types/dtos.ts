// Hand-mirrored TypeScript shapes of the Rust DTOs in
// `src-tauri/src/contracts/dtos.rs`. Field names match serde's default
// (snake_case, same as the Rust struct fields) — no renaming at this
// boundary, so a diff on one side is easy to spot on the other.

export interface ImagePreviewDto {
	width: number;
	height: number;
	/** PNG bytes, base64-encoded. Downscaled for original/result previews
	 *  sent over IPC — export always re-encodes the full-resolution image
	 *  held in the backend, never this thumbnail. */
	png_base64: string;
}

export interface BackgroundRemovalResultDto {
	original: ImagePreviewDto;
	result: ImagePreviewDto;
}

export interface ExportResultDto {
	/** `null` means the user cancelled the save dialog — not an error. */
	saved_path: string | null;
}
