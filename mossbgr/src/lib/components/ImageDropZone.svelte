<script lang="ts">
	let {
		busy,
		onPick
	}: {
		busy: boolean;
		onPick: () => void;
	} = $props();
</script>

<div class="card">
	<h2 class="h4">Choose an image</h2>
	<p>Pick a PNG or JPEG to remove its background. Processing happens entirely on this device.</p>
	<button class="button button-primary" onclick={onPick} disabled={busy}>
		{busy ? 'Working…' : 'Choose Image…'}
	</button>
</div>
