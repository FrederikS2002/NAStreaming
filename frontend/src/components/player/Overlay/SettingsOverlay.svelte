<script lang="ts">
	import { videoSubscribe } from '../stores/video';
	//TODO:onDestory
	let video: HTMLVideoElement | null;
	videoSubscribe((value) => {
		video = value.video;
	});
	$: checkMute = () => {
		if (video) {
			return video.muted;
		}
		return false;
	};
	const toggleMute = () => {
        if(!video) return;
		video.muted = !video.muted;
	};

	$: getVolume = () => {
		if (video) {
			return video.volume * 100;
		}
		return 100;
	};

	const setPlaybackDefault = () => {
        if(!video) return;
		video.playbackRate = 1;
	};
</script>

<div class="settings">
	<div class="settings_overlay">
		<!-- TODO:video, sound, Subtitles, skiptime-->
	</div>
	<div class="playback" on:click={setPlaybackDefault}>
		<svg
			stroke="currentColor"
			fill="currentColor"
			stroke-width="0"
			viewBox="0 0 24 24"
			height="1em"
			width="1em"
			xmlns="http://www.w3.org/2000/svg"
			><path fill="none" d="M0 0h24v24H0z" /><path
				d="M20.38 8.57l-1.23 1.85a8 8 0 01-.22 7.58H5.07A8 8 0 0115.58 6.85l1.85-1.23A10 10 0 003.35 19a2 2 0 001.72 1h13.85a2 2 0 001.74-1 10 10 0 00-.27-10.44zm-9.79 6.84a2 2 0 002.83 0l5.66-8.49-8.49 5.66a2 2 0 000 2.83z"
			/></svg
		>
	</div>
	<div class="volume" on:click={toggleMute}>
		{#if checkMute()}
			<svg
				stroke="currentColor"
				fill="none"
				stroke-width="2"
				viewBox="0 0 24 24"
				stroke-linecap="round"
				stroke-linejoin="round"
				height="1em"
				width="1em"
				xmlns="http://www.w3.org/2000/svg"
				><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" /><line
					x1="23"
					y1="9"
					x2="17"
					y2="15"
				/><line x1="17" y1="9" x2="23" y2="15" /></svg
			>
		{:else if getVolume() == 0}
			<svg
				stroke="currentColor"
				fill="none"
				stroke-width="2"
				viewBox="0 0 24 24"
				stroke-linecap="round"
				stroke-linejoin="round"
				height="1em"
				width="1em"
				xmlns="http://www.w3.org/2000/svg"
				><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" /></svg
			>
		{:else if getVolume() <= 50}
			<svg
				stroke="currentColor"
				fill="none"
				stroke-width="2"
				viewBox="0 0 24 24"
				stroke-linecap="round"
				stroke-linejoin="round"
				height="1em"
				width="1em"
				xmlns="http://www.w3.org/2000/svg"
				><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" /><path
					d="M15.54 8.46a5 5 0 0 1 0 7.07"
				/></svg
			>
		{:else}
			<svg
				stroke="currentColor"
				fill="none"
				stroke-width="2"
				viewBox="0 0 24 24"
				stroke-linecap="round"
				stroke-linejoin="round"
				height="1em"
				width="1em"
				xmlns="http://www.w3.org/2000/svg"
				><polygon points="11 5 6 9 2 9 2 15 6 15 11 19 11 5" /><path
					d="M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07"
				/></svg
			>
		{/if}
	</div>
</div>

<style lang="scss">
	.settings {
		z-index: 2;
		position: absolute;
		display: flex;
		right: 0;
		top: 0;
		margin: 2%;
		color: white;
	}
</style>
