<script lang="ts">
	import { skipBackward, skipForward } from '../stores/video';
	import SkipPrevious from 'svelte-material-icons/SkipPreviousCircle.svelte';
	import SkipNext from 'svelte-material-icons/SkipNextCircle.svelte';
</script>

<div class="back" on:click={skipBackward}>
	<SkipPrevious />
</div>
<div class="forward" on:click={skipForward}>
	<SkipNext />
</div>

<style lang="scss">
	div {
		z-index: 2;
		position: absolute;
		top: 50%;
		color: white;
		transform: translate(-50%, -50%);

		&.back {
			left: 30%;
		}

		&.forward {
			right: 30%;
		}
	}
</style>
