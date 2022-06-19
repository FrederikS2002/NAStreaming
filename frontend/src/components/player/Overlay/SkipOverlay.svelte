<script lang="ts">
	import { skipBackward, skipForward } from '../stores/video';
	export let overlay: Boolean;
</script>

<div class="left" on:dblclick={skipBackward} on:click|self={() => (overlay = !overlay)} />
<div class="right" on:click|self={() => (overlay = !overlay)} on:dblclick={skipForward} />

<style lang="scss">
	div {
		position: absolute;
		z-index: 1;
		width: 50%;
		height: 100%;
		top: 0;
		&.left {
			left: 0;
		}
		&.right {
			right: 0;
		}
	}
</style>
