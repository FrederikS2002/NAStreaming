<script lang="ts">
	import './index.scss';
	import { generateStyle, getEpisodeData } from './index';
	import type { episodeOrder } from '../../types';
	export let fml: number;
	export let episode: episodeOrder;
	let episodedata = getEpisodeData(episode.epi, episode.type);
	let pieheight: number;
</script>

{#if episodedata != null}
	<div class="episode" style={generateStyle(fml)} on:click>
		<div class="image">
			{#if episode.type == 'epi'}
				<img class="thumb" src={episodedata.thumb} alt="" draggable="false" />
			{:else if episode.type == 'file'}
				<div
					class="pie"
					bind:clientHeight={pieheight}
					style="width: {pieheight}px; background: conic-gradient(green 0deg 1deg, #fff 1deg 360deg);"
				/>
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
		width: 200px;
	}
</style>
