<script lang="ts">
	import './index.scss';
	import { getEpisodeData } from './index';
	import type { test } from '../stores/episodeOrder';
	export let episode: test;
	export let pos: number;
	let episodedata = getEpisodeData(episode.epi);
</script>

{#if episodedata != null}
	<div class="episode" style={`top:${pos}px`}>
		<div class="image"><img class="thumb" src={episodedata.thumb} alt="" draggable="false" /></div>
		<div class="text">
			<h1>{episode.new}.</h1>
			<h1>{episodedata.title}</h1>
			<h2>{episodedata.description}</h2>
		</div>
	</div>
{/if}

<style lang="scss">
	.episode {
		position: absolute;
		opacity: 0.5;
	}
</style>
