<script>
	import Video from './Video.svelte';
	import Overlay from './Overlay/Overlay.svelte';
	import Fullscreen from 'svelte-fullscreen';
	import './videoplayer.scss'
</script>

<div class="player">
	<Fullscreen>
		<Overlay />
		<Video />
	</Fullscreen>
</div>
