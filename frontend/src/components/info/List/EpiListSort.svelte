<script lang="ts">
	import Episode from './Episode/edit.svelte';
	import EpisodeGhost from './Episode/EpisodeGhost.svelte';
	import { generateFML } from './Episode/index';
	import { epi_order_subscribe } from './stores/episodeOrder';
	import { subscribedrag } from './stores/drag';
	import type { episodeOrder } from '../types';
	let posY: number;
	let dragstuff: any;
	let epilist: episodeOrder[];

	epi_order_subscribe((value): episodeOrder[] => (epilist = value));
	subscribedrag((value) => (dragstuff = value));

	$: active = () => {
		for (let i = 0; i < dragstuff.cords.length; i++) {
			if (dragstuff.cords[i] && dragstuff.cords[i] > posY) {
				return i;
			}
		}
		for (let i = dragstuff.cords.length - 1; i > 0; i--) {
			if (dragstuff.cords[i]) {
				return i;
			}
		}
		return null;
	};

	const setPosY = (e: any) => {
		posY = e.clientY + dragstuff.scrollprogress;
	};

	$: console.log(epilist);

	//TODO: FIGURE OUT HOW TO FIX THE SPACING BUG
</script>

<div class="episodes" on:mousemove={setPosY}>
	{#if dragstuff.dragStartIndex != null}
		<EpisodeGhost episode={epilist[1]} pos={posY} />
	{/if}

	{#each epilist as episode, i}
		<Episode
			{episode}
			fml={generateFML(i, epilist.length)}
			pos={posY}
			activeEpi={dragstuff.dragStartIndex != null ? active() : null}
		/>
	{/each}
</div>
