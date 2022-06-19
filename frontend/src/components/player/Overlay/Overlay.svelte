<script lang="ts">
	import NextEpisodeOverlay from './NextEpisodeOverlay.svelte';
	import SkipButtonsOverlay from './SkipButtonsOverlay.svelte';
	import TimeOverlay from './TimeOverlay.svelte';
	import SettingsOverlay from './SettingsOverlay.svelte';
	import ModesOverlay from './ModesOverlay.svelte';
	import PlayPauseButtonOverlay from './PlayPauseButtonOverlay.svelte';
	import { videoSubscribe } from '../stores/video';
	import SkipOverlay from './SkipOverlay.svelte';
	import Slider from './Slider.svelte';

	let overlay = true;
	let video;
	//TODO:CHECK ON DESTROY
	const videodel = videoSubscribe((value) => (video = value.video));
	const toggleOverlay = () => (overlay = !overlay);
</script>

<div on:click|self={() => (overlay = !overlay)}>
	{#if overlay}
		<ModesOverlay />
		<SettingsOverlay />
		<TimeOverlay />
		<SkipButtonsOverlay />
		{#if video}
			<Slider />
		{/if}

		<NextEpisodeOverlay />
	{/if}
	<PlayPauseButtonOverlay {overlay} />
	<SkipOverlay {overlay} />
</div>

<style lang="scss">
	div {
		position: absolute;
		width: 100%;
		height: 100%;
	}
</style>
