<script lang="ts">
	import type { ImagePreviewDto } from '../types/dtos';

	let {
		original,
		result,
		busy,
		onRemoveBackground,
		onChooseAnother
	}: {
		original: ImagePreviewDto;
		result: ImagePreviewDto | null;
		busy: boolean;
		onRemoveBackground: () => void;
		onChooseAnother: () => void;
	} = $props();
</script>

<div class="card-grid">
	<div class="card">
		<div class="card-header">Original</div>
		<img class="img-fluid" src="data:image/png;base64,{original.png_base64}" alt="Original" />
	</div>
	<div class="card">
		<div class="card-header">Background removed</div>
		{#if result}
			<img
				class="img-fluid checkerboard"
				src="data:image/png;base64,{result.png_base64}"
				alt="Background removed"
			/>
		{:else}
			<p class="small">Run background removal to see the result here.</p>
		{/if}
	</div>
</div>

<div class="preview-actions">
	<button class="button button-secondary" onclick={onChooseAnother} disabled={busy}>
		Choose another image
	</button>
	<button class="button button-primary" onclick={onRemoveBackground} disabled={busy}>
		{busy ? 'Removing background…' : result ? 'Run again' : 'Remove background'}
	</button>
</div>

<style>
	.preview-actions {
		display: flex;
		justify-content: flex-end;
		gap: 0.75rem;
		margin: 1rem 0 1.5rem;
	}

	/* Makes transparency in the result preview visible against the card. */
	.checkerboard {
		background-image:
			linear-gradient(45deg, #ccc 25%, transparent 25%),
			linear-gradient(-45deg, #ccc 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #ccc 75%),
			linear-gradient(-45deg, transparent 75%, #ccc 75%);
		background-size: 16px 16px;
		background-position:
			0 0,
			0 8px,
			8px -8px,
			-8px 0;
	}
</style>
