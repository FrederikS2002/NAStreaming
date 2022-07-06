<script lang="ts">
	import Select from 'svelte-select';
	import TagInput from './TagInput.svelte';
	import TextInput from './TextInput.svelte';
	import { items } from './movietypelist';
	import { createMovie } from './createMovie';

	let titles: string[] | undefined;
	let categories: string[] | undefined;
	let type_: { value: string; label: string } | null;
	let age_restriction: string | undefined;
	let cover: FileList | undefined;
	let error: undefined | string;

	$: if (cover && !cover[0].type.includes('image/')) {
		cover = undefined;
	}

	$: submit_btn = () => {
		if (
			titles == undefined ||
			titles.length == 0 ||
			categories == undefined ||
			categories.length == 0 ||
			type_ == null ||
			age_restriction == undefined ||
			age_restriction == '' ||
			cover == undefined
		) {
			return 'submit disabled';
		}
		return 'submit';
	};

	const submit = async (_: any) => {
		const req = await createMovie(titles, categories, type_, age_restriction, cover);
		if (req && req.success) {
			//TODO: clear
			error = undefined;
		} else {
			error = req.data;
		}
	};
</script>

<div class="menu">
	{#if error} <div class="error">{error}</div> {/if}
	<div class="text">
		<TagInput placeholder="Names" bind:value={titles} />
		<TagInput placeholder="Categories" bind:value={categories} />
		<div class="oneline">
			<div class="parts">
				<TextInput number={true} placeholder="age restriction" bind:value={age_restriction} />
			</div>
			<div class="themed parts"><Select {items} bind:value={type_} /></div>
		</div>
		<div class={submit_btn()} on:click={submit}>Submit</div>
	</div>
	<div class="preview"><input type="file" bind:files={cover} /></div>
</div>

<style lang="scss">
	@import './addmovie.scss';
</style>
