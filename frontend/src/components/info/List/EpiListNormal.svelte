<script lang="ts">
	import Episode from './Episode/Redirect.svelte';
	import { generateFML } from './Episode';
	import { epi_order_subscribe, type test } from './stores/episodeOrder';
	let epilist: test[];
	epi_order_subscribe((value): test[] => (epilist = value));
</script>

<div class="episodes">
	{#each epilist as episode, i}
		<Episode {episode} fml={generateFML(i, epilist.length)} />
	{/each}
</div>
