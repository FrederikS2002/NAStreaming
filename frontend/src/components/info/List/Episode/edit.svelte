<script lang="ts">
	import './index.scss';
	export let episode: test;
	export let fml: number;
	export let pos: number;
	export let activeEpi: number | null;
	export let style: string = '';
	import { generateStyle } from '.';
	import { onMouseDown, onMouseUp } from './edit';
	import { updateTurnPoints } from '../stores/store';
	import type { test } from '../stores/episodeOrder';
	import { epi_sub, type test2 } from '../stores/episodeData';
	import { onDestroy } from 'svelte/internal';

	let me: HTMLElement;
	$: top = me ? me.getBoundingClientRect().top + window.scrollY : 0;
	$: height = me ? me.clientHeight : 0;
	$: if (activeEpi == null) {
		updateTurnPoints(episode.new, top);
	}

	const oorb = (top: number, height: number, pos: number) => {
		if (activeEpi == episode.new) {
			if (pos < top + height / 2) {
				return `margin-top: ${height}px`;
			} else {
				return `margin-bottom: ${height}px`;
			}
		}
		return '';
	};

	let episodedata: test2 | null;
	onDestroy(epi_sub((v) => (episodedata = v.filter((v) => v.epi === episode.epi)[0])));
</script>

{#if episodedata != null}
	<div
		class="episode"
		style={generateStyle(fml) + ';' + oorb(top, height, pos) + ';' + style}
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
