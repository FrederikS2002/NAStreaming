<script lang="ts">
import { onDestroy } from 'svelte';

import { file_location, url } from '../urls';
import { epi_data_sub } from './stores/request';

	import { bindVideoData, videoSubscribe } from './stores/video';
import type { EpisodeData } from './types';
	import './video.scss';

	let currentTime = 0;
	let duration = 0;
	let paused = true;
	let video: any = null;

	//TODO: FIX UPDATE time -1h
	$: bindVideoData(video, duration, currentTime, paused);
	videoSubscribe((value) => (currentTime = value.currentTime));
	let data: EpisodeData;
	onDestroy(epi_data_sub((value) => (data = value)));
</script>

<video
	bind:currentTime
	bind:duration
	bind:paused
	bind:this={video}
	poster=""
	preload="metadata"
>
	<source src={url + file_location + data.movie + "/" +data.file} type="video/mp4" />
	Your browser does not support the video tag.
	<track kind="captions" />
</video>
