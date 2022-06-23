<script lang="ts">
	import './index.scss';
	import { generateStyle, getEpisodeData } from '.';
	import { onMouseDown } from './edit';
	import { updateTurnPoints } from '../stores/store';
	import type { test } from '../stores/episodeOrder';
	import { epi_sub, type test2 } from '../stores/episodeData';
	import { onDestroy } from 'svelte';
	import { oorb } from './index';

	export let episode: test;
	export let fml: number;
	export let pos: number;
	export let activeEpi: number | null;
	export let style: string = '';

	let me: HTMLElement;
	let episodedata = getEpisodeData(episode.epi);

	$: top = me ? me.getBoundingClientRect().top + window.scrollY : 0;
	$: height = me ? me.clientHeight : 0;
	$: if (activeEpi == null) {
		updateTurnPoints(episode.new, top);
	}
</script>

{#if episodedata != null}
	<div
		class="episode"
		style={generateStyle(fml) + ';' + oorb(top, height, pos, activeEpi == episode.new) + ';' + style}
		draggable={true}
		bind:this={me}
		on:mousedown={(_) => {
			me.classList.add('dragged');
			onMouseDown(episode.new);
		}}
		on:drop={(_) => _}
	>
		<div class="image"><img class="thumb" src={episodedata.thumb} alt="" draggable="false" /></div>
		<div class="text">
			<h1>{episode.new}.</h1>
			<h1>{episodedata.title}</h1>
			<h2>{episodedata.description}</h2>
		</div>
	</div>
{/if}
