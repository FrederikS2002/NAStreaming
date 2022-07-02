<script lang="ts">
	import Select from 'svelte-select';
	import TagInput from './TagInput.svelte';
	import TextInput from './TextInput.svelte';

	let titles: string[] | undefined;
	let categories: string[] | undefined;
	let type_: { value: string; label: string } | null;
	let age_restriction: string | undefined;
	let cover: FileList | undefined;

	let items = [
		{ value: 'movie', label: 'Movie' },
		{ value: 'series', label: 'Series' },
		{ value: 'special', label: 'Special' }
	];
	export async function makeRequest() {
		let data = new FormData();
		if (titles) data.append('titles', mkRd(titles));
		if (categories) data.append('categories', mkRd(categories));
		if (type_) data.append('type', type_.value);
		if (age_restriction) data.append('age_restriction', saveNum(age_restriction));
		if (cover && cover[0]) data.append('cover', cover[0], 'test.jpeg');
		fetch('http://127.0.0.1:8080/create_movie', {
			method: 'post',
			//make sure to serialize your JSON body
			body: data
		})
			.then((res) => res.json())
			.then((data) => {
				console.log(data);
			})
			.catch((err) => {
				console.error('Error: ', err);
			});
	}

	const mkRd = (stringy: string[]) =>
		',' + stringy.map((s) => s.replaceAll(/,/g, '&comma;')).join(',') + ',';
	const saveNum = (num: string) => num.replaceAll('[^d]', '');
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
