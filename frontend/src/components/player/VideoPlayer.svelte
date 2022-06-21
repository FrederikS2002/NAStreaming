<script>
	import Video from './Video.svelte';
	import Overlay from './Overlay/Overlay.svelte';
	import Fullscreen from 'svelte-fullscreen';
</script>

<div>
	<Fullscreen let:onToggle={toggleFullscreen}>
		<Overlay />
		<Video />
	</Fullscreen>
</div>

<style lang="scss">
	div {
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		background: black;
	}
</style>
