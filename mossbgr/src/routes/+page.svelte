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
	<h1 class="h2">Mossy Background Remover</h1>

	<ErrorBanner message={imageStore.error} onDismiss={dismissError} />

	{#if !imageStore.original}
		<ImageDropZone busy={imageStore.busy} onPick={() => imageStore.pickImage()} />
	{:else}
		<BeforeAfterPreview
			original={imageStore.original}
			result={imageStore.result}
			busy={imageStore.busy}
			onRemoveBackground={() => imageStore.removeBackground()}
			onChooseAnother={() => imageStore.reset()}
		/>
		<ExportButton
			disabled={!imageStore.result}
			busy={imageStore.busy}
			savedPath={imageStore.savedPath}
			onExport={() => imageStore.exportResult()}
		/>
	{/if}
</main>

<style>
	main {
		max-width: 900px;
		margin: 0 auto;
		padding: 1.5rem;
	}
</style>
