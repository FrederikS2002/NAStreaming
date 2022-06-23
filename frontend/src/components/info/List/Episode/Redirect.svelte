<script lang="ts">
	import Episode from './normal.svelte';
	import { redirect } from '../../../tools';
	import type { test } from '../stores/episodeOrder';
	import { generateUrl } from '.';

	export let episode: test;
	export let fml: number = 1;
</script>

<Episode {episode} {fml} on:click={(_) => redirect(generateUrl(episode.new))} />
