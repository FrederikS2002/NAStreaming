<script lang="ts">
	import EditSvg from './editSVG.svelte';
	import { startUpload } from "../upload";
	import './normal.scss';
	export let uuid: string;
</script>

<div class="options">
	<div class="play" />
	<div class="edit" on:click={e => startUpload(uuid)}>
		<EditSvg />
	</div>
	<div class="seasons" />
</div>
