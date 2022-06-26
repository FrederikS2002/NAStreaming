<script context="module" lang="ts">
	import { url } from '../../components/urls';

	export async function load({ fetch, params }: any) {
		const id = params.id;
		let backend = `${url}/movie_detail/${id}`;

		const res = await fetch(backend);
		const data = await res.json();
		if (res.ok) {
			return {
				props: { movies: data }
			};
		} else {
			return {
				props: { movies: null }
			};
		}
	}
</script>

<script lang="ts">
	import { onMouseUp } from '../../components/info/List/Episode/edit';
	import Info from '../../components/info/Info/index.svelte';
	import { onDropFile } from '../../components/info/upload';
	import type { RootObj } from '../../components/info/types';
	import { setData } from '../../components/info/List/stores/baseData';
	export let movies: RootObj;
	setData({
		uuid: movies.uuid,
		titles: movies.titles,
		icon: movies.icon,
		thumb: movies.thumb,
		color: movies.color,
		description: movies.description
	});
	const onMouseUpL = () => {
		let dragged = document.querySelector('.dragged');
		if (dragged) {
			dragged.classList.remove('dragged');
		}
		onMouseUp();
	};
</script>

<div
	class="container"
	on:mouseup={(_) => onMouseUpL()}
	on:dragover={(e) => e.preventDefault()}
	on:drop={(e) => onDropFile(e)}
>
	<Info {movies} />
</div>

<style lang="scss">
	.container {
		z-index: 0;
		position: relative;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 1);
	}
</style>
