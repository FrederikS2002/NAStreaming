<script lang="ts">
	import Select from 'svelte-select';
	import TagInput from './TagInput.svelte';
	import TextInput from './TextInput.svelte';
	import { items } from './movietypelist';

	let titles: string[] | undefined;
	let categories: string[] | undefined;
	let type_: { value: string; label: string } | null;
	let age_restriction: string | undefined;
	let cover: FileList | undefined;
</script>

<div class="menu">
	<TagInput placeholder="Names" bind:value={titles} />
	<TagInput placeholder="Categories" bind:value={categories} />
	<TextInput number={true} placeholder="age restriction" bind:value={age_restriction} />
	<div class="themed"><Select {items} bind:value={type_} /></div>
	<!-- <input type="file" bind:files={cover} /> -->
	<!-- <button on:click={makeRequest}>Submit</button> -->
</div>

<style lang="scss">
	@import 'addmovie.scss';
</style>
