<script lang="ts">
	import './index.scss';
	import { generateStyle, getEpisodeData } from './index';
	import type { test } from '../stores/episodeOrder';
	export let fml: number;
	export let episode: test;
	let episodedata = getEpisodeData(episode.epi);
</script>

{#if episodedata != null}
	<div class="episode" style={generateStyle(fml)} on:click>
		<div class="image"><img class="thumb" src={episodedata.thumb} alt="" draggable="false" /></div>
		<div class="text">
			<h1>{episode.new}.</h1>
			<h1>{episodedata.title}</h1>
			<h2>{episodedata.description}</h2>
		</div>
	</div>
{/if}
