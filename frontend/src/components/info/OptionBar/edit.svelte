<script lang="ts">
	import DoneSvg from './svgs/done.svelte';
	export let uuid: string;
	export let single: boolean = false;
	export let related: boolean = true;
	export let mode: string = single ? 'details' : 'episodes';
	export let edit: boolean = false;
	//TODO: admin

	const save = (new_mode: string) => {
		//TODO: save changes
		mode = new_mode;
	};
</script>

<div class="options">
	<div class="left">
		{#if !single}
			<div class="episodes" on:click={(_) => save('episodes')}>Episodes</div>
		{/if}
		<div class="details" on:click={(_) => save('details')}>Details</div>
		{#if related}
			<div class="related" on:click={(_) => save('related')}>Related</div>
		{/if}
	</div>
	<div class="right">
		<div class="upload" on:click={(_) => (edit = false)}>
			<DoneSvg />
		</div>
	</div>
</div>

<style lang="scss">
	@import 'normal.scss';
</style>
