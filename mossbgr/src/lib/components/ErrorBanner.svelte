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
	<div class="alert alert-danger alert-dismissible error-toast" role="alert">
		{message}
		{#if onDismiss}
			<button class="close" onclick={onDismiss} aria-label="Dismiss">&times;</button>
		{/if}
	</div>
{/if}

<style>
	/* Mossy app convention: error boxes appear at the bottom-right of the
	   screen. Fixed, so showing or hiding one never shifts the page layout. */
	.error-toast {
		position: fixed;
		right: 1rem;
		bottom: 1rem;
		z-index: var(--z-modal-backdrop);
		max-width: min(28rem, calc(100vw - 2rem));
		margin: 0;
		box-shadow: var(--shadow-lg);
		overflow-wrap: anywhere;
	}
</style>
