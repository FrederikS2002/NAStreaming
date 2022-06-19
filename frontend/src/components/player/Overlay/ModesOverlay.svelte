<script lang="ts">
	import { videoSubscribe } from '../stores/video';
	import FullscreenIcon from 'svelte-bootstrap-icons/lib/Fullscreen/Fullscreen.svelte';
	import FullscreenExitIcon from 'svelte-bootstrap-icons/lib/FullscreenExit/FullscreenExit.svelte';

	const checkPiP = () => {
		try {
			return 'pictureInPictureEnabled' in document;
		} catch (e) {
			return false;
		}
	};
	let video: HTMLVideoElement | null;
	videoSubscribe((value) => (video = value.video));
	const togglePiP = () => {
		try {
			if (video && video !== document.pictureInPictureElement) {
				video.requestPictureInPicture();
			} else {
				document.exitPictureInPicture();
			}
		} catch (error) {
			console.error(error);
		}
	};
	let fullscreen = false;
</script>

<div class="modes">
	<div class="back">
		<svg
			stroke="currentColor"
			fill="currentColor"
			stroke-width="0"
			viewBox="0 0 512 512"
			height="1em"
			width="1em"
			xmlns="http://www.w3.org/2000/svg"
		>
			<path
				fill="none"
				stroke-linecap="round"
				stroke-linejoin="round"
				stroke-width="48"
				d="M328 112L184 256l144 144"
			/>
		</svg>
	</div>
	{#if checkPiP()}
		<div class="pip" on:click={togglePiP}>
			<svg
				stroke="currentColor"
				fill="none"
				stroke-width="0"
				viewBox="0 0 24 24"
				height="1em"
				width="1em"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path d="M20 12H14V17H20V12Z" fill="currentColor" />
				<path
					fill-rule="evenodd"
					clip-rule="evenodd"
					d="M1 6C1 4.89543 1.89543 4 3 4H21C22.1046 4 23 4.89543 23 6V18C23 19.1046 22.1046 20 21 20H3C1.89543 20 1 19.1046 1 18V6ZM3 6H21V18H3L3 6Z"
					fill="currentColor"
				/>
			</svg>
		</div>
	{/if}
	<!-- TODO:ADD FULLSCREEN -->
	<div
		class="fullscreen"
		on:click={() => {
			fullscreen = !fullscreen;
		}}
	>
		{#if !fullscreen}
			<FullscreenIcon />
		{:else}
			<FullscreenExitIcon />
		{/if}
	</div>
</div>

<style lang="scss">
	.modes {
		z-index: 2;
		position: absolute;
		display: flex;
		left: 0;
		top: 0;
		margin: 2%;
		color: white;
	}
</style>
