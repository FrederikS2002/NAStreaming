<script lang="ts">
	import { loop_guard } from 'svelte/internal';
	import './icon.scss';
	export let color: number[];
	export let active = false;
	import Selector from './Selector.svelte';
	let data: any | HTMLDivElement;
</script>

{#if active}
	<div
		class="obj"
		style={active ? `--secondary-color: rgb(${color[0]} ${color[1]} ${color[2]})` : ''}
		bind:this={data}
		on:click
	>
		<slot />
	</div>
	{#if data && data.childNodes[0]}
		<Selector
			offset={data.offsetTop + data.childNodes[0].clientHeight / 2}
			width={data.childNodes[0].getBoundingClientRect().left * 0.85}
		    style={active ? `--secondary-color: rgb(${color[0]} ${color[1]} ${color[2]})` : ''}
		/>
	{/if}
{:else}
	<div
		class="obj"
		style={active ? `--secondary-color: rgb(${color[0]} ${color[1]} ${color[2]})` : ''}
		on:click
	>
		<slot />
	</div>
{/if}
