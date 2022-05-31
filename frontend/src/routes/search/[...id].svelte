<script context="module">
	export async function load({ fetch, params }) {
        const ids = params.id;
        if(ids.split('/').length > 1) {
            return {props:{}}
        }
        let backend;
        if(ids.length > 0){
            backend = `http://127.0.0.1:8080/search_movie/${ids}/1/10`;
        }else{
            backend = `http://127.0.0.1:8080/search_movie/1/100`;
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
	export let movies;
</script>

<SearchMovieMenu {movies} />
