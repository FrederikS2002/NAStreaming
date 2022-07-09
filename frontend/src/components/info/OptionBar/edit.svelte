<script lang="ts">
	import { setEditMode } from '../store';

	import DoneSvg from './svgs/done.svelte';
	export let single: boolean = false;
	export let related: boolean = true;

	export let mode: string = single ? 'details' : 'episodes';

	export let save: any;

	const update = async (new_mode: string) => {
		await save(mode);
		mode = new_mode;
	};
	const close = async () => {
		await save(mode);
		setEditMode(false);
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
		<div class="upload" on:click={(_) => close()}>
			<DoneSvg />
		</div>
	</div>
</div>

<style lang="scss">
	@import 'normal.scss';
</style>
