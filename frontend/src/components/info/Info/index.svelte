<script lang="ts">
	import { setEpisodeData } from '../List/stores/episodeData';
	import { setEpisodeOrderArr } from '../List/stores/episodeOrder';
	import { generateOrder, onScroll } from '.';
	import OptionBarNormal from '../OptionBar/normal.svelte';
	import OptionBarEdit from '../OptionBar/edit.svelte';
	import Details from '../Details.svelte';
	import Related from '../Related.svelte';
	import EpisodesEdit from '../EpisodesEdit.svelte';
	import DetailsEdit from '../DetailsEdit.svelte';
	import RelatedEdit from '../RelatedEdit.svelte';
	import Episodes from '../episodes.svelte';

	export let movies: any;
	let mode: string;
	let edit: boolean = false;

	$: console.log(mode, edit);

	let top: HTMLImageElement | null;
	$: top_height = top ? top.clientHeight : 0;

	setEpisodeData(movies.epilist);
	setEpisodeOrderArr(generateOrder(movies.epilist));
</script>

<div class="thumb" on:scroll={onScroll}>
	<img src={movies.thumb} alt="" bind:this={top} />
	<div class="over_thumb" style={'height: ' + top_height + 'px'}>
		<img src={movies.icon} alt="" draggable="false" />
		<div class="infos" />
		<div class="categories" />
		<div class="play" />
		<div class="other">
			<div class="trailer" />
			<div class="fav" />	
		</div>
	</div>
</div>
<div class="content">
	{#if !edit}
		<OptionBarNormal bind:mode bind:edit />
	{:else}
		<OptionBarEdit uuid={movies.uuid} bind:mode bind:edit />
	{/if}

	{#if mode}
		{#if edit}
			{#if mode == 'episodes'}
				<EpisodesEdit />
			{:else if mode == 'details'}
				<DetailsEdit />
			{:else if mode == 'related'}
				<RelatedEdit />
			{/if}
		{:else if mode == 'episodes'}
			<Episodes />
		{:else if mode == 'details'}
			<Details />
		{:else if mode == 'related'}
			<Related />
		{/if}
	{/if}
</div>

<style lang="scss">
	@import 'index.scss';
</style>
