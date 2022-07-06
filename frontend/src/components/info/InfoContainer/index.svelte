<script lang="ts">
	import Info from '../Info/index.svelte';
	import { onDropFile } from '../upload';
	import { onMouseUpL } from './index';
	import { setData } from '../List/stores/baseData';
	import type { RootObj } from '../types';
	import { edit_mode_sub } from '../store';

	export let movies: RootObj;
	setData({
		uuid: movies.uuid,
		titles: movies.titles,
		icon: movies.icon,
		thumb: movies.thumb,
		color: movies.color,
		description: movies.description
	});
	let edit = false;
	edit_mode_sub((v) => (edit = v));
</script>

{#if edit}
	<div
		class="sub_container"
		on:mouseup={onMouseUpL}
		on:dragover={(e) => e.preventDefault()}
		on:drop={onDropFile}
	>
		<Info {movies} />
	</div>
{:else}
	<div class="sub_container">
		<Info {movies} />
	</div>
{/if}

<style lang="scss">
	@import 'index.scss';
</style>
