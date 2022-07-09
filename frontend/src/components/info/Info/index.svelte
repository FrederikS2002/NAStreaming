<script lang="ts">
	import { setEpisodeData } from '../List/stores/episodeData';
	import { setEpisodeOrderArr } from '../List/stores/episodeOrder';
	import { generateOrder, onScroll } from '.';
	import OptionBarNormal from '../OptionBar/normal.svelte';
	import OptionBarEdit from '../OptionBar/edit.svelte';
	import Details from '../Details.svelte';
	import Related from '../Related.svelte';
	import DetailsEdit from '../DetailsEdit.svelte';
	import RelatedEdit from '../RelatedEdit.svelte';
	import EpiListNormal from '../List/EpiListNormal.svelte';
	import EpiListSort from '../List/EpiListSort.svelte';
	import { edit_mode_sub } from '../store';
	import { startUpload } from '../upload';
	import type { RootObj } from '../types';
	import Upload from './upload.svelte';

	export let movies: RootObj;
	let mode: string;
	let edit: boolean = false;
	edit_mode_sub((v) => (edit = v));

	$: console.log(mode, edit);

	let top: HTMLImageElement | null;
	$: top_height = top ? top.clientHeight : 0;

	const save = async (old_mode: string) => {
		console.log('test');
		if (old_mode == 'episodes') {
			await startUpload(movies.uuid);
		} else if (old_mode == 'details') {
		} else if (old_mode == 'related') {
		}
	};
	setEpisodeData(movies.epilist);
	setEpisodeOrderArr(generateOrder(movies.epilist));
</script>

{#if movies.icon && movies.thumb}
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
{:else}
	<div class="thumb_upload">
		<Upload uuid={movies.uuid} classs="upload_thumb_btn" />
		<Upload uuid={movies.uuid} classs="upload_icon_btn" />
		<Upload uuid={movies.uuid} classs="add_trailer" />
	</div>
{/if}
<div class="content">
	{#if !edit}
		<OptionBarNormal bind:mode />
	{:else}
		<OptionBarEdit bind:mode {save} />
	{/if}

	{#if mode}
		{#if edit}
			{#if mode == 'episodes'}
				<EpiListSort />
			{:else if mode == 'details'}
				<DetailsEdit />
			{:else if mode == 'related'}
				<RelatedEdit />
			{/if}
		{:else if mode == 'episodes'}
			<EpiListNormal />
		{:else if mode == 'details'}
			<Details />
		{:else if mode == 'related'}
			<Related />
		{/if}
	{/if}
</div>

<style lang="scss">
	@import 'index.scss';
	.thumb_upload {
		width: 100%;
		height: 100%;
	}
</style>
