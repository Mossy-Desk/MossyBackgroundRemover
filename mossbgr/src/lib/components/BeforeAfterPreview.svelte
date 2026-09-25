<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { ImagePreviewDto } from '../types/dtos';

	let {
		original,
		result,
		busy,
		onRemoveBackground,
		onChooseAnother,
		children
	}: {
		original: ImagePreviewDto;
		result: ImagePreviewDto | null;
		busy: boolean;
		onRemoveBackground: () => void;
		onChooseAnother: () => void;
		/** Extra actions rendered at the end of the action bar (e.g. export). */
		children?: Snippet;
	} = $props();
</script>

<div class="card frame">
	<div class="panels">
		<figure class="panel">
			<figcaption>Original</figcaption>
			<div class="image-area">
				<img src="data:image/png;base64,{original.png_base64}" alt="Original" />
			</div>
		</figure>
		<figure class="panel">
			<figcaption>Background removed</figcaption>
			<div class="image-area checkerboard">
				{#if result}
					<img src="data:image/png;base64,{result.png_base64}" alt="Background removed" />
				{:else}
					<p class="placeholder">Run background removal to see the result here.</p>
				{/if}
			</div>
		</figure>
	</div>

	<div class="actions">
		<button class="button button-link" onclick={onChooseAnother} disabled={busy}>
			Back to file selection
		</button>
		<div class="actions-main">
			<button class="button button-primary" onclick={onRemoveBackground} disabled={busy}>
				{busy ? 'Removing background…' : result ? 'Run again' : 'Remove background'}
			</button>
			{@render children?.()}
		</div>
	</div>
</div>

<style>
	/* The frame takes all the height the page gives it; the two panels take
	   what's left above the action bar. */
	.frame {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
		margin-bottom: 0;
		padding: 1.5rem;
		min-height: 24rem;
	}

	.panels {
		flex: 1;
		display: grid;
		grid-template-columns: 1fr 1fr;
		gap: 1.25rem;
		min-height: 0;
	}

	@media (max-width: 700px) {
		.panels {
			grid-template-columns: 1fr;
		}
	}

	.panel {
		display: flex;
		flex-direction: column;
		gap: 0.6rem;
		margin: 0;
		min-height: 14rem;
	}

	figcaption {
		text-align: center;
		font-weight: var(--font-weight-black);
		color: var(--mossy-green);
	}

	.image-area {
		position: relative;
		flex: 1;
		min-height: 0;
		background: #fdfbf8; /* off-white, deliberately outside the MossyGrave palette for now */
		border: 2px solid var(--mossy-green-lighter);
		border-radius: var(--border-radius-lg);
		overflow: hidden;
	}

	/* The image fills the box and is letterboxed, never cropped or stretched. */
	.image-area img {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: contain;
	}

	.placeholder {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		margin: 0;
		padding: 1rem;
		text-align: center;
		color: var(--mossy-wood);
	}

	/* Makes transparency in the result visible across the whole panel. */
	.checkerboard {
		background-color: #fdfbf8;
		background-image:
			linear-gradient(45deg, #ececec 25%, transparent 25%),
			linear-gradient(-45deg, #ececec 25%, transparent 25%),
			linear-gradient(45deg, transparent 75%, #ececec 75%),
			linear-gradient(-45deg, transparent 75%, #ececec 75%);
		background-size: 16px 16px;
		background-position:
			0 0,
			0 8px,
			8px -8px,
			-8px 0;
	}

	.actions {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		gap: 0.75rem;
	}

	.actions-main {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		gap: 0.75rem;
	}
</style>
