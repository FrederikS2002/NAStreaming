<script lang="ts">
	import MovieCard from './MovieCard.svelte';
	export let movies: any;
</script>

<div class="movies">
	{#if movies}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
		{#each movies as movie}
			<MovieCard {movie} />
		{/each}
	{:else}
		<text>No results</text>
	{/if}
</div>

<style lang="scss">
	@import 'search.scss';
</style>
