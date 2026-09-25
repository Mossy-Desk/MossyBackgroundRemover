<script lang="ts">
	import '$lib/styles/vendor/mossy-grave.css';
	import ErrorBanner from '$lib/components/ErrorBanner.svelte';
	import ImageDropZone from '$lib/components/ImageDropZone.svelte';
	import BeforeAfterPreview from '$lib/components/BeforeAfterPreview.svelte';
	import ExportButton from '$lib/components/ExportButton.svelte';
	import { imageStore } from '$lib/stores/image.svelte';

	function dismissError() {
		imageStore.error = null;
	}
</script>

<main>
	<ErrorBanner message={imageStore.error} onDismiss={dismissError} />

	{#if !imageStore.original}
		<ImageDropZone
			busy={imageStore.busy}
			onPick={() => imageStore.pickImage()}
			onDrop={(path) => imageStore.loadFromDrop(path)}
		/>
	{:else}
		<BeforeAfterPreview
			original={imageStore.original}
			result={imageStore.result}
			busy={imageStore.busy}
			onRemoveBackground={() => imageStore.removeBackground()}
			onChooseAnother={() => imageStore.reset()}
		>
			<ExportButton
				disabled={!imageStore.result}
				busy={imageStore.busy}
				savedPath={imageStore.savedPath}
				onExport={() => imageStore.exportResult()}
			/>
		</BeforeAfterPreview>
	{/if}
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		justify-content: center; /* content sits in the middle of the window, not stuck to the top */
		gap: 1.25rem;
		min-height: 100vh;
		max-width: min(1200px, 100%);
		margin: 0 auto;
		/* Breathing room that scales with the window. */
		padding: clamp(1.5rem, 6vh, 4rem) clamp(1rem, 4vw, 3rem);
	}
</style>
