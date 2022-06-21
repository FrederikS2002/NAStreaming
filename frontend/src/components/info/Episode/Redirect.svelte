<script lang="ts">
	export let episode: any;
	export let fml: number = 1;
    export let url: string;
	import Episode from './index.svelte';
    import {redirect} from '../../tools';
</script>

<Episode {episode} {fml} on:click={_ => redirect(url)}/>
