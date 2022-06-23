<script lang="ts">
	import EpiListSort from '../List/EpiListSort.svelte';
	import OptionBarNormal from '../OptionBar/normal.svelte';
	import './index.scss';
	import { setEpisodeData } from '../List/stores/episodeData';
	import { setEpisodeOrderArr } from '../List/stores/episodeOrder';
	import { generateOrder, onScroll } from '.';

	export let movies: any;

	let top: HTMLImageElement | null;
	$: top_height = top ? top.clientHeight : 0;

	setEpisodeData(movies.epilist);
	setEpisodeOrderArr(generateOrder(movies.epilist));
</script>

<div
	class="info"
	on:scroll={onScroll}
	style="background: rgb({movies.color[0]},{movies.color[1]},{movies.color[2]})"
>
	<img class="thumb" src={movies.thumb} alt="" bind:this={top} />
	<div class="over_thumb" style={'height: ' + top_height + 'px'}>
		<img src={movies.icon} alt="" draggable="false" />
		<h2>{movies.description}</h2>
	</div>
	<OptionBarNormal />
	<EpiListSort />
</div>
