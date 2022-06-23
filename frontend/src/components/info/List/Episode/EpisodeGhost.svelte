<script lang="ts">
	import './index.scss';
	export let episode: any;
    export let pos: number;
	import {generateStyle} from './index';
</script>

<div class="episode" style={`top:${pos}px`}>
	<div class="image"><img class="thumb" src={episode.thumb} alt="" draggable="false" /></div>
	<div class="text">
		<h1>{episode.changed}.</h1>
		<h1>{episode.title}</h1>
		<h2>{episode.description}</h2>
	</div>
</div>

<style  lang="scss">
    .episode {
        position: absolute;
        opacity: .5;
    }
</style>