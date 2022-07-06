<script lang="ts">
	import EditSvg from './svgs/edit.svelte';
	import AddSvg from './svgs/add.svelte';

	export let single: boolean = false;
	export let related: boolean = true;
	export let mode: string = single ? 'details' : 'episodes';
	export let edit: boolean = false;
	//TODO: admin
	let admin = true;
</script>

<div class="options">
	<div class="left">
		{#if !single}
			<div class="episodes" on:click={(_) => (mode = 'episodes')}>Episodes</div>
		{/if}
		<div class="details" on:click={(_) => (mode = 'details')}>Details</div>
		{#if related}
			<div class="related" on:click={(_) => (mode = 'related')}>Related</div>
		{/if}
	</div>
	<div class="right">
		{#if admin}
			<div class="add" on:click={(_) => (edit = true)}>
				<AddSvg />
			</div>
			<div class="edit" on:click={(_) => (edit = true)}>
				<EditSvg />
			</div>
		{/if}
		<div class="seasons" />
	</div>
</div>

<style lang="scss">
	@import 'normal.scss';
</style>
