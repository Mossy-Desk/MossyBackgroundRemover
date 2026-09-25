// Thin wrapper around @tauri-apps/api — the only file in the frontend that
// talks to Tauri IPC directly. One typed function per backend command.
// Everything else in the frontend goes through this module rather than
// calling `invoke` itself.
import { invoke } from '@tauri-apps/api/core';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { getCurrentWebview } from '@tauri-apps/api/webview';
import type { BackgroundRemovalResultDto, ExportResultDto, ImagePreviewDto } from './types/dtos';

export async function pickAndLoadImage(): Promise<ImagePreviewDto | null> {
	return await invoke<ImagePreviewDto | null>('pick_and_load_image');
}

export async function loadImageFromDrop(path: string): Promise<ImagePreviewDto> {
	return await invoke<ImagePreviewDto>('load_image_from_drop', { path });
}

export async function removeBackground(): Promise<BackgroundRemovalResultDto> {
	return await invoke<BackgroundRemovalResultDto>('remove_background');
}

export async function exportResult(): Promise<ExportResultDto> {
	return await invoke<ExportResultDto>('export_result');
}

export interface FileDropHandlers {
	/** True while files are dragged over the window, false once they leave or are dropped. */
	onHover: (hovering: boolean) => void;
	/** Called with the first dropped file's path (one image at a time in v1). */
	onDrop: (path: string) => void;
}

export function onFileDrop(handlers: FileDropHandlers): Promise<UnlistenFn> {
	return getCurrentWebview().onDragDropEvent((event) => {
		const payload = event.payload;
		if (payload.type === 'enter') {
			handlers.onHover(true);
		} else if (payload.type === 'leave') {
			handlers.onHover(false);
		} else if (payload.type === 'drop') {
			handlers.onHover(false);
			const [first] = payload.paths;
			if (first) handlers.onDrop(first);
		}
	});
}
