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

<button class="button button-success" onclick={onExport} disabled={disabled || busy}>
	Export as PNG&hellip;
</button>
{#if savedPath}
	<span class="small saved">Saved to {savedPath}</span>
{/if}

<style>
	.saved {
		overflow-wrap: anywhere;
	}
</style>
