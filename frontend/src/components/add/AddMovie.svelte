<script lang="ts">
import TagInput from './TagInput.svelte';

	import TextInput from './TextInput.svelte';
	let titles: string[] | undefined;
	let categories: string[] | undefined;;
	let type_: string;
	let age_restriction: string;
	let cover: any;

	export async function makeRequest() {
		let data = new FormData();
		if (titles) data.append('titles', mkRd(titles));
		if (categories) data.append('categories', mkRd(categories));
		if (type_) data.append('type', type_);
		if (age_restriction) data.append('age_restriction', age_restriction);
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

	const mkRd = (stringy: string[]) => {
		return ","+stringy.map((s) => s.replace(/,/g, '&comma;')).join(',')+",";
	};
</script>

<div class="menu">
	<TagInput placeholder="Names" bind:value={titles} />
	<TagInput placeholder="Categories" bind:value={categories} />
	<TextInput placeholder="Type" bind:value={type_} />
	<TextInput number={true} placeholder="age restriction" bind:value={age_restriction} />
	<!-- <input type="file" bind:files={cover} /> -->
	<!-- <button on:click={makeRequest}>Submit</button> -->
</div>
