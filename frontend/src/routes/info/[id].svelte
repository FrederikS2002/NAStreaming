<script context="module" lang="ts">
	import { url } from '../../components/urls';

	export async function load({ fetch, params }: any) {
		const id = params.id;
		let backend = `${url}/movie_detail/${id}`;

		const res = await fetch(backend);
		const data = await res.json();
		if (res.ok) {
			return {
				props: { movies: data }
			};
		} else {
			return {
				props: { movies: null }
			};
		}
	}
</script>

<script lang="ts">
	import type { RootObj } from '../../components/info/types';
	export let movies: RootObj;
	import InfoContainer from '../../components/info/InfoContainer/index.svelte';
</script>

<InfoContainer {movies} />
