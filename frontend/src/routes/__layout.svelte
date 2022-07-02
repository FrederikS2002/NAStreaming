<script lang="ts">
	import Services from '../components/ui/Sidebar/Services.svelte';
	import Background from '../components/ui/Background/index.svelte';
	import { page } from '$app/stores';
</script>

{#if $page.routeId === 'play/[...id]'}
	<slot />
{:else}
	<Services />
	<Background><slot /></Background>
{/if}
