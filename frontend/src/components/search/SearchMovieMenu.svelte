<script lang="ts">
	import MovieCard from './MovieCard.svelte';
	export let movies;
</script>

<div class="movies">
	{#if movies}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
	{:else}
		<text>No results</text>
	{/if}
</div>

<style lang="scss">
	.movies {
		width: 100%;
		height: 100%;
		display: grid;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		grid-column-gap: 1rem;
		grid-row-gap: 2rem;
		overflow: auto !important;
	}
</style>
