<script context="module" lang="ts">
	import {url} from '../../components/urls';
	export async function load({ fetch, params }: any) {
		const ids = params.id;
		if (ids.split('/').length > 1) {
			return { props: {} };
		}
		let backend;
		if (ids.length > 0) {
			backend = `${url}/search_movie/${ids}/1/10`;
		} else {
			backend = `${url}/search_movie/1/100`;
		}

		const res = await fetch(backend);
		const data = await res.json();
		if (res.ok) {
			return {
				props: { movies: data }
			};
		}
	}
</script>

<script lang="ts">
	import SearchMovieMenu from '../../components/search/SearchMovieMenu.svelte';
	export let movies: any;
</script>

<SearchMovieMenu {movies} />
