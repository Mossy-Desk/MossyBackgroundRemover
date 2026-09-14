// Thin wrapper around @tauri-apps/api — the only file in the frontend that
// talks to Tauri IPC directly. One typed function per backend command.
// Everything else in the frontend goes through this module rather than
// calling `invoke` itself.
import { invoke } from '@tauri-apps/api/core';
import type { BackgroundRemovalResultDto, ExportResultDto, ImagePreviewDto } from './types/dtos';

export async function pickAndLoadImage(): Promise<ImagePreviewDto | null> {
	return await invoke<ImagePreviewDto | null>('pick_and_load_image');
}

export async function removeBackground(): Promise<BackgroundRemovalResultDto> {
	return await invoke<BackgroundRemovalResultDto>('remove_background');
}

export async function exportResult(): Promise<ExportResultDto> {
	return await invoke<ExportResultDto>('export_result');
}
