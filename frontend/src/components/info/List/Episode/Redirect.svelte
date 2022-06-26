<script lang="ts">
	import Episode from './normal.svelte';
	import { generateUrl } from '.';
	import { goto } from '$app/navigation';
	import type { episodeOrder } from '../../types';

	export let episode: episodeOrder;
	export let fml: number = 1;
</script>

<Episode {episode} {fml} on:click={(_) => goto(generateUrl(episode.new))} />
