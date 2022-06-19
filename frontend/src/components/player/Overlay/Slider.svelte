<script>
	import { videoSubscribe } from '../stores/video';

	let video;
	//TODO:CHECK ON DESTROY
	const videodel = videoSubscribe((value) => (video = value.video));
</script>

<div class="play" orient="vertical">
	<input type="range" min="0.5" max="5" step="0.1" bind:value={video.playbackRate} />
</div>
<div class="vol" orient="vertical">
	<input
		type="range"
		min="0"
		max="100"
		value={video.volume * 100}
		on:change={(e) => (video.volume = e.target.value / 100)}
	/>
</div>

<style lang="scss">
	div {
		position: absolute;
		z-index: 2;
		top: 10%;
		right: 0;
		transform: rotate(270deg);
		&.vol {
			right: 2%;
		}
	}
</style>
