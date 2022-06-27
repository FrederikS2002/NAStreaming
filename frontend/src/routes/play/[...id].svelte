<script context="module" lang="ts">
	import { epi_data, url } from '../../components/urls';
	export async function load({ fetch, params }: any) {
		const ids = params.id;
		const res = await fetch(url + epi_data + ids);
		const data = await res.json();
		if (res.ok) {
			return {
				props: { data: data }
			};
		}
	}
</script>

<script lang="ts">
	// let time = 0;
	// let next_update = 0;
	// $: if (time >= next_update) {
	// 	//TODO: PUSH Progress
	// 	next_update = time + 5;
	// }

	// function handleKeydown(event) {
	// 	let key = event.key;
	// 	switch (key) {
	// 		case 'ArrowUp':
	// 			videoVolumeUp();
	// 			break;
	// 		case 'ArrowDown':
	// 			videoVolumeDown();
	// 			break;
	// 		case 'ArrowLeft':
	// 			skipBackward();
	// 			break;
	// 		case 'ArrowRight':
	// 			skipForward();
	// 			break;
	// 		case 'f':
	// 			//TODO:Fix Keybind fullscreen
	// 			//toggleFullscreen();
	// 			break;
	// 		case 'p':
	// 			if (checkPiP()) {
	// 				togglePiP();
	// 			}
	// 			break;
	// 		case 'm':
	// 			toggleMute();
	// 			break;
	// 		case ' ':
	// 			togglePlay();
	// 			break;
	// 		case 'c':
	// 			//TODO:ToggleCaptions
	// 			break;
	// 	}
	// }
	import VideoPlayer from '../../components/player/VideoPlayer.svelte';
	import type { EpisodeData } from '../../components/player/types';
	import { setVideoData } from '../../components/player/stores/request';
	export let data: EpisodeData;
	setVideoData(data);
</script>

<!--<svelte:window on:keydown={handleKeydown}/>-->
<VideoPlayer />
