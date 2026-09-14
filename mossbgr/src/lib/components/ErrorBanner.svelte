<script lang="ts">
	let {
		message,
		onDismiss
	}: {
		message: string | null;
		onDismiss?: () => void;
	} = $props();
</script>

{#if message}
	<div class="alert alert-danger alert-dismissible">
		{message}
		{#if onDismiss}
			<button class="close" onclick={onDismiss} aria-label="Dismiss">&times;</button>
		{/if}
	</div>
{/if}
