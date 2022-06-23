<script lang="ts">
	import Episode from './normal.svelte';
	import { redirect } from '../../../tools';
	import type { test } from '../stores/episodeOrder';
	import { url, play } from '../../../urls';
	import { onDestroy } from 'svelte';
	import { epi_sub } from '../stores/episodeData';

	export let episode: test;
	export let fml: number = 1;
	const generateUrl = () => {
		let uuid: string = '';
		onDestroy(epi_sub((v) => (uuid = v.filter((v) => v.epi === episode.epi)[0].uuid)));
		console.log(uuid);
		return url + play + uuid;
	};
</script>

<Episode {episode} {fml} on:click={(_) => redirect(generateUrl())} />
