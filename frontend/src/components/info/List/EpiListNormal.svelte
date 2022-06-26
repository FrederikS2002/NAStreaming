<script lang="ts">
	import Episode from './Episode/Redirect.svelte';
	import { generateFML } from './Episode';
	import { epi_order_subscribe } from './stores/episodeOrder';
import type { episodeOrder } from '../types';
	let epilist: episodeOrder[];
	epi_order_subscribe((value): episodeOrder[] => (epilist = value));
</script>

<div class="episodes">
	{#each epilist as episode, i}
		<Episode {episode} fml={generateFML(i, epilist.length)} />
	{/each}
</div>
