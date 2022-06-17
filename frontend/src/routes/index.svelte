<script context="module">
    import url from "../components/urls"
    let backend = url+'/search_movie/1/10';
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
    import PlusButton from "../components/add/PlusButton.svelte";
    import AddMovie from "../components/add/AddMovie.svelte";
	export let movies;
    let addMovie = false;
    const toggleAddMovie = () => {
        addMovie = !addMovie;
    };
</script>
<PlusButton on:click={toggleAddMovie}/>
{#if addMovie}
    <AddMovie on:click={toggleAddMovie}/>
{/if}
<SearchMovieMenu {movies} />
