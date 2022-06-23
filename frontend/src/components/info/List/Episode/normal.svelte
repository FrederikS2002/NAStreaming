<script lang="ts">
	import './index.scss';
	import { generateStyle } from './index';
	import { onDestroy } from 'svelte';
	import type { test } from '../stores/episodeOrder';
	import { epi_sub, type test2 } from '../stores/episodeData';
	export let fml: number;
	export let episode: test;
	let episodedata: test2 | null;
	onDestroy(epi_sub((v) => (episodedata = v.filter((v) => v.epi === episode.epi)[0])));
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
