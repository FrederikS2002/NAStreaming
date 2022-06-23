<script context="module" lang="ts">
	import { url } from '../../components/urls';

	export async function load({ fetch, params }) {
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
	export let movies: any;

	const onMouseUpL = () => {
		let dragged = document.querySelector('.dragged');
		if (dragged) {
			dragged.classList.remove('dragged');
		}
		onMouseUp();
	};
</script>

<div class="container" on:mouseup={(_) => onMouseUpL()}>
	<Info {movies} />
	<!--    <div on:dragover={(ev) => { ev.preventDefault() }} on:drop={e => onDropFile(e)} class="uplaodcontainer">-->
	<!--        {#each array as epi, index}-->
	<!--            <Episode epi={index} title={epi.title} image={epi.img_src} time={epi.time}/>-->
	<!--        {/each}-->
	<!--        <HiddenEpisode epi={array.length} />-->
	<!--    </div>-->
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
