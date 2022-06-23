<script lang="ts">
	import './index.scss';
	export let episode: any;
	export let fml: number;
	export let pos: number;
	export let activeEpi: number | null;
	export let style: string = '';
	import { generateStyle } from '.';
	import { onMouseDown, onMouseUp } from './edit';
	import { updateTurnPoints } from '../stores/store';
	let me: HTMLElement;
	$: top = me ? me.getBoundingClientRect().top + window.scrollY : 0;
	$: height = me ? me.clientHeight : 0;
	$: if (activeEpi == null) {
		updateTurnPoints(episode.changed, top);
	}

	const oorb = (top: number, height: number, pos: number) => {
		if (activeEpi == episode.changed) {
			if (pos < top + height / 2) {
				return `margin-top: ${height}px`;
			} else {
				return `margin-bottom: ${height}px`;
			}
		}
		return '';
	};
</script>

<div
	class="episode"
	style={generateStyle(fml) + ';' + oorb(top, height, pos) + ';' + style}
	draggable={true}
	bind:this={me}
	on:mousedown={(_) => {
		me.classList.add('dragged');
		onMouseDown(episode.changed);
	}}
	on:drop={(_) => _}
>
	<div class="image"><img class="thumb" src={episode.thumb} alt="" draggable="false" /></div>
	<div class="text">
		<h1>{episode.changed}.</h1>
		<h1>{episode.title}</h1>
		<h2>{episode.description}</h2>
	</div>
</div>
