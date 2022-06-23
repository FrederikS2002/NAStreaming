<script lang="ts">
	import './index.scss';
	export let episode: any;
	export let fml: number;
	import {generateStyle} from './index';
</script>

<div class="episode" style={generateStyle(fml)} on:click>
	<div class="image"><img class="thumb" src={episode.thumb} alt="" draggable="false" /></div>
	<div class="text">
		<h1>{episode.changed}.</h1>
		<h1>{episode.title}</h1>
		<h2>{episode.description}</h2>
	</div>
</div>
