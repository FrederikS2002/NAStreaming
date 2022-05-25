<script context="module">
	let backend = 'http://127.0.0.1:8080/search_movie/1/10';
	export async function load({ fetch }) {
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
	import SearchMovieMenu from '../components/search/SearchMovieMenu.svelte';
	export let movies;
	console.log(movies);
</script>

<SearchMovieMenu {movies} />
