import * as api from '../api';
import type { ImagePreviewDto } from '../types/dtos';

class ImageStore {
	original: ImagePreviewDto | null = $state(null);
	result: ImagePreviewDto | null = $state(null);
	busy: boolean = $state(false);
	error: string | null = $state(null);
	savedPath: string | null = $state(null);

	async pickImage() {
		this.busy = true;
		try {
			const picked = await api.pickAndLoadImage();
			if (picked === null) return; // user cancelled — not an error
			this.original = picked;
			this.result = null;
			this.savedPath = null;
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.busy = false;
		}
	}

	async removeBackground() {
		if (!this.original) return;
		this.busy = true;
		try {
			const removal = await api.removeBackground();
			this.original = removal.original;
			this.result = removal.result;
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.busy = false;
		}
	}

	async exportResult() {
		if (!this.result) return;
		this.busy = true;
		try {
			const exported = await api.exportResult();
			this.savedPath = exported.saved_path; // null = user cancelled the save dialog
			this.error = null;
		} catch (err) {
			this.error = String(err);
		} finally {
			this.busy = false;
		}
	}

	reset() {
		this.original = null;
		this.result = null;
		this.savedPath = null;
		this.error = null;
	}
}

export const imageStore = new ImageStore();
