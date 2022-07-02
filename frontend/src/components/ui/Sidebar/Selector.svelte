<script lang="ts">
	import Selector from './svgs/selector.svelte';
	export let width: number;
	export let offset: number;
	export let style: string;
</script>

<div class="selector" style="--width: {width}px; --offset: {offset}px;{style}" >
	<Selector />
    <div class="glow" />
</div>

<style lang="scss">
    @import 'selector.scss';
</style>