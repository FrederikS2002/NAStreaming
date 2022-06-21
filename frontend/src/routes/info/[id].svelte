<script context="module" lang="ts">
	import { url } from '../../components/urls';
	import OptionBarNormal from '../../components/info/OptionBar/normal.svelte';

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
	import EpiListNormal from '../../components/info/Episode/EpiListNormal.svelte';
	import Episode from '../../components/info/Episode/index.svelte';
	export let movies: any;
	let top: HTMLImageElement | null;
	$: top_height = top ? top.clientHeight : 0;

	let edit = false;
	const toggleEdt = () => {
		edit = !edit;
	};
</script>

<div class="container">
	<div class="info" style="background: rgb({movies.color[0]},{movies.color[1]},{movies.color[2]})">
		<img class="thumb" src={movies.thumb} alt="" bind:this={top} />
		<div class="over_thumb" style={'height: ' + top_height + 'px'}>
			<img src={movies.icon} alt="" draggable="false" />
			<h2>{movies.description}</h2>
		</div>
		<OptionBarNormal />
		<EpiListNormal epilist={movies.epilist} />
	</div>
	<!--    <div on:dragover={(ev) => { ev.preventDefault() }} on:drop={e => onDropFile(e)} class="uplaodcontainer">-->
	<!--        {#each array as epi, index}-->
	<!--            <Episode epi={index} title={epi.title} image={epi.img_src} time={epi.time}/>-->
	<!--        {/each}-->
	<!--        <HiddenEpisode epi={array.length} />-->
	<!--    </div>-->
</div>

<!-- <style lang="scss">
	.uplaodcontainer {
		position: absolute;
		top: 10%;
		left: 10%;
		width: 80%;
		height: 80%;
	}
</style> -->
<style lang="scss">
	.container {
		z-index: 0;
		position: relative;
		width: 100%;
		height: 100%;
		background-color: rgba(0, 0, 0, 1);
	}
	.info {
		z-index: 1;
		position: fixed;
		width: 70%;
		height: 100%;
		left: 15%;
		overflow-x: auto;
		& .thumb {
			z-index: 2;
			position: absolute;
			width: 100%;
			object-fit: cover;
		}
		& .over_thumb {
			z-index: 3;
			position: relative;
			width: 100%;
			color: white;
			& img {
				margin: 5% 0 0 5%;
				width: 30%;
			}
		}
	}

	.episodes {
		z-index: 4;
		position: relative;
		width: 100%;
		height: 110vh;
	}
</style>
