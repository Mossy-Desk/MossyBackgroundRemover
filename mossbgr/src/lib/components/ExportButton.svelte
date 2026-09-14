<script lang="ts">
	let {
		disabled,
		busy,
		savedPath,
		onExport
	}: {
		disabled: boolean;
		busy: boolean;
		savedPath: string | null;
		onExport: () => void;
	} = $props();
</script>

<div class="export-row">
	<button class="button button-success" onclick={onExport} disabled={disabled || busy}>
		Export as PNG&hellip;
	</button>
	{#if savedPath}
		<span class="small">Saved to {savedPath}</span>
	{/if}
</div>

<style>
	.export-row {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		margin-bottom: 1.5rem;
	}
</style>
