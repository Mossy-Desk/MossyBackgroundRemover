<script lang="ts">
	import { onFileDrop } from '../api';

	let {
		busy,
		onPick,
		onDrop
	}: {
		busy: boolean;
		onPick: () => void;
		onDrop: (path: string) => void;
	} = $props();

	let hovering = $state(false);

	// The subscription is async, so the component can be destroyed before it
	// resolves. `disposed` makes that late listener get removed immediately
	// instead of leaking (each leak would fire one extra load per drop).
	$effect(() => {
		let disposed = false;
		let unlisten: (() => void) | undefined;

		onFileDrop({
			onHover: (isHovering) => (hovering = isHovering),
			onDrop: (path) => {
				if (!busy) onDrop(path);
			}
		}).then((remove) => {
			if (disposed) remove();
			else unlisten = remove;
		});

		return () => {
			disposed = true;
			unlisten?.();
		};
	});
</script>

<h2 class="title">Drag your file</h2>
<div class="drop-target" class:hovering>
	<button class="button button-link" onclick={onPick} disabled={busy}>
		{busy ? 'Working…' : 'or open file explorer'}
	</button>
</div>

<style>
	.title {
		margin: 0;
		text-align: center;
		font-family: var(--font-heading);
		font-weight: var(--font-weight-black);
		font-size: clamp(1.5rem, 3vw, 2.25rem);
		color: var(--mossy-green);
	}

	.drop-target {
		flex: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		min-height: 14rem;
		max-height: 34rem;
		background: #fdfbf8; /* off-white, deliberately outside the MossyGrave palette for now */
		border: 2px solid var(--mossy-green-lighter);
		border-radius: var(--border-radius-lg);
		color: var(--mossy-green);
		transition:
			background var(--transition-fast),
			border-color var(--transition-fast);
	}

	.drop-target.hovering {
		border-color: var(--mossy-green);
		background: #f4f7e6;
	}
</style>
