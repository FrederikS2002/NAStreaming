<script lang="ts">
	export let epilist: any[];
	import Episode from './index.svelte';
	import { generateFML } from './index';
</script>

<div class="episodes">
	{#each epilist as episode, i}
		<Episode {episode} fml={generateFML(i, epilist.length)} />
	{/each}
</div>
