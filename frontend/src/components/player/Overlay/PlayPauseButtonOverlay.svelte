<script lang="ts">
	import { videoSubscribe } from '../stores/video';
	import Play from 'svelte-bootstrap-icons/lib/Play/Play.svelte';
	import Pause from 'svelte-bootstrap-icons/lib/Pause/Pause.svelte';

	export let overlay: any;
	let video: HTMLVideoElement | null;
	let paused: boolean;

	videoSubscribe((value) => {
		video = value.video;
		paused = value.paused;
	});

	$: isPlaying = () => {
		if (paused) {
			return false;
		}
		return true;
	};

	const togglePlay = () => {
		if (!video) return;
		if (isPlaying()) {
			video.pause();
		} else {
			video.play();
		}
	};
</script>

<div class={overlay ? '' : 'hidden'} on:click={() => togglePlay()}>
	{#if isPlaying()}
		<Pause />
	{:else}
		<Play />
	{/if}
</div>

<style lang="scss">
	div {
		z-index: 2;
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		color: white;

		&.hidden {
			color: transparent;
		}
	}
</style>
