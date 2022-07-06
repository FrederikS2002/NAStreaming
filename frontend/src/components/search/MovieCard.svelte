<script lang="ts">
	import { cover_location, url } from '../urls';
	import type { movie } from './types';
	export let movie: movie;
	let title = movie.titles.split(",")[1];
</script>

<a sveltekit:noscroll sveltekit:prefetch href={'/info/' + movie.uuid}
	><div class="card">
		<img src={url + cover_location + movie.uuid + '.jpeg'} alt="" />
		<div class="title">
			<h1>{title}</h1>
		</div>
	</div></a
>

<style lang="scss">
	@import 'movieCard.scss';
</style>
