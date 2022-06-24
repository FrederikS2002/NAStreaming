<script lang="ts">
	import './index.scss';
	import { generateStyle, getEpisodeData } from '.';
	import { onMouseDown } from './edit';
	import { subscribedrag, updateTurnPoints } from '../stores/store';
	import type { test } from '../stores/episodeOrder';
	import { oorb } from './index';

	export let episode: test;
	export let fml: number;
	export let pos: number;
	export let activeEpi: number | null;
	export let style: string = '';

	let me: HTMLElement;
	$: episodedata = getEpisodeData(episode.epi, episode.type);

	$: top = me ? me.getBoundingClientRect().top + window.scrollY : 0;
	$: height = me ? me.clientHeight : 0;
	$: if (activeEpi == null) {
		updateTurnPoints(episode.new, top);
	}
	let test: any;
	let pieheight: number;
	subscribedrag((v) => (test = v));
</script>

{#if episodedata != null}
	<div
		class="episode"
		id={episode.epi ? episode.epi.toString() : 'bo'}
		style={generateStyle(fml) +
			';' +
			oorb(episode.new, top, height, pos, activeEpi == episode.new, test.dragOverIndex) +
			';' +
			style}
		draggable={true}
		bind:this={me}
		on:mousedown={(_) => {
			me.classList.add('dragged');
			onMouseDown(episode.new);
		}}
		on:drop={(_) => _}
	>
		<div class="image">
			{#if episode.type == 'epi'}
				<img class="thumb" src={episodedata.thumb} alt="" draggable="false" />
			{:else if episode.type == 'file'}
				<div class="pie" bind:clientHeight={pieheight} style="width: {pieheight}px; background: conic-gradient(green 0deg 1deg, #fff 1deg 360deg);" />
			{:else}
				<img class="thumb" src="" alt="" draggable="false" />
			{/if}
		</div>
		<div class="text">
			<h1>{episode.new}.</h1>
			<h1>{episodedata.title}</h1>
			<h2>{episodedata.description}</h2>
		</div>
	</div>
{/if}

<style lang="scss">
	.pie {
		position: relative;
		height: 100%;
		border-radius: 50%;
	}
</style>
