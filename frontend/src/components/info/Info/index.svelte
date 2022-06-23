<script lang="ts">
	import EpiListSort from '../List/EpiListSort.svelte';
	import OptionBarNormal from '../OptionBar/normal.svelte';
	import { setScrollProgress } from '../List/stores/store';
	import './index.scss';
	import { setEpisodeData, epi_sub } from '../List/stores/episodeData';
	import { setEpisodeOrderArr } from '../List/stores/episodeOrder';
	export let movies: any;

	let top: HTMLImageElement | null;
	$: top_height = top ? top.clientHeight : 0;
	const onScroll = (e: any) => {
		setScrollProgress(e.target.scrollTop);
	};

	setEpisodeData(movies.epilist);
	const generateOrder = () => {
		let result: any[] = [];
		for (let i = 0; i < movies.epilist.length; i++) {
			result.push({
				epi: movies.epilist[i].epi,
				new: movies.epilist[i].epi,
				type: 'epi'
			});
		}
		return result.sort((a, b) => a.new - b.new);
	};
	setEpisodeOrderArr(generateOrder());
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
