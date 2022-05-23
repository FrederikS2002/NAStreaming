<script lang="ts">
	$: playing = false;
	$: overlay = true;
	$: fullscreen = false;
	$: remainingTime = true;
	import Play from 'svelte-bootstrap-icons/lib/Play/Play.svelte';
	import Pause from 'svelte-bootstrap-icons/lib/Pause/Pause.svelte';
	import FullscreenIcon from 'svelte-bootstrap-icons/lib/Fullscreen/Fullscreen.svelte';
	import FullscreenExitIcon from 'svelte-bootstrap-icons/lib/FullscreenExit/FullscreenExit.svelte';
	import SkipPrevious from 'svelte-material-icons/SkipPreviousCircle.svelte';
	import SkipNext from 'svelte-material-icons/SkipNextCircle.svelte';
	import Fullscreen from 'svelte-fullscreen';

	let time = 0;
	let next_update = 0;
	let duration;
	let video;
	let hover;
	let time1;
	let time2;
    let toggleFullscreen;
	$: calculatePixel = () => {
		if (hover && time1 && time2) {
			return hover.clientWidth - (time1.offsetWidth + time2.offsetWidth + 10) + 'px';
		}
		return 0;
	};

	$: if (time >= next_update) {
		//TODO: PUSH Progress
		next_update = time + 5;
	}

	const checkPiP = () => {
		try {
			return 'pictureInPictureEnabled' in document;
		} catch (e) {
			return false;
		}
	};

	const togglePiP = () => {
		try {
			if (video !== document.pictureInPictureElement) {
				video.requestPictureInPicture();
			} else {
				document.exitPictureInPicture();
			}
		} catch (error) {
			console.error(error);
		}
	};

	const togglePlay = () => {
		if (playing == true) {
			video.pause();
		} else {
			video.play();
		}
		playing = !playing;
	};
	const toggleMute = () => {
		video.muted = !video.muted;
	}

	const videoVolumeUp = () => {
		video.volume += 5;
	}
	const videoVolumeDown = () => {
		video.volume -= 5;
	}
	const skipForward = () => {
		time += 5;
	};

	const skipBackward = () => {
		time -= 5;
	};
    function handleKeydown(event) {
        let key = event.key;
        switch (key){
            case "ArrowUp":
                videoVolumeUp()
                break;
            case "ArrowDown":
                videoVolumeDown()
                break;
            case "ArrowLeft":
                skipBackward();
                break;
            case "ArrowRight":
                skipForward();
                break;
            case "f":
                //TODO:Fix Keybind fullscreen
                //toggleFullscreen();
                break;
            case "p":
                if(checkPiP()){
                    togglePiP();
                }
                break;
            case "m":
                toggleMute();
                break;
            case " ":
                togglePlay();
                break;
            case "c":
                //TODO:ToggleCaptions
                break;
        }
        //console.log(key);
    }

	$: getTime = (time) => {
		if (isNaN(time)) {
			time = 0;
		}
		let date = new Date(null);
		date.setSeconds(Math.round(time)); // specify value for SECONDS here
		let h = date.getHours() - 1;
		let m = date.getMinutes();
		let s = date.getSeconds();
		const checkTime = (value) => {
			if (value < 10) {
				return '0' + value;
			}
			return value;
		};
		// add a zero in front of numbers<10
		m = checkTime(m);
		s = checkTime(s);
		let result = '';
		if (h > 0) {
			result += checkTime(h) + ':';
		}
		result += m + ':' + s;
		return result;
	};

</script>
<svelte:window on:keydown={handleKeydown}/>
<div class="content">
	<Fullscreen let:onToggle={toggleFullscreen}>
		<div class="overlay" on:click|self={() => (overlay = !overlay)}>
			{#if overlay}
				<div class="skipb" on:click={skipBackward}><SkipPrevious /></div>
				<div class="skipf" on:click={skipForward}><SkipNext /></div>
			{/if}

			<div class="playpause {overlay ? '' : 'hidden'}" on:click={togglePlay}>
				{#if playing}
					<Pause />
				{:else}
					<Play />
				{/if}
			</div>

			<div class="skip left" on:dblclick={skipBackward} on:click|self={() => (overlay = !overlay)} />
			<div class="skip right" on:click|self={() => (overlay = !overlay)} on:dblclick={skipForward} />
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
						><path
							fill="none"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="48"
							d="M328 112L184 256l144 144"
						/></svg
					>
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
							><path d="M20 12H14V17H20V12Z" fill="currentColor" /><path
								fill-rule="evenodd"
								clip-rule="evenodd"
								d="M1 6C1 4.89543 1.89543 4 3 4H21C22.1046 4 23 4.89543 23 6V18C23 19.1046 22.1046 20 21 20H3C1.89543 20 1 19.1046 1 18V6ZM3 6H21V18H3L3 6Z"
								fill="currentColor"
							/></svg
						>
					</div>
				{/if}

				<div
					class="fullscreen"
					on:click={() => {
						fullscreen = !fullscreen;
						toggleFullscreen();
					}}
				>
					{#if !fullscreen}
						<FullscreenIcon />
					{:else}
						<FullscreenExitIcon />
					{/if}
				</div>
			</div>
			<div class="settings">
				<div class="sound" />
				<div class="playback" />
				<div class="settings_overlay">
					<!-- video, sound, Subtitles, skiptime-->
				</div>
			</div>
			<div class="hover" bind:this={hover}>
				<!--TODO:Make responsive -->
				<time bind:this={time1}>{getTime(time)}</time>
				<input
					type="range"
					min="0"
					max={duration * 1000}
					style="width: {calculatePixel()}"
					value={time * 1000}
					on:input={(e) => (time = e.target.value / 1000)}
				/>
				<time bind:this={time2}
					>{remainingTime ? '-' + getTime(duration - time) : getTime(duration)}</time
				>
			</div>
		</div>

		<video
			preload="metadata"
			bind:this={video}
			bind:currentTime={time}
			bind:duration
			poster="https://sveltejs.github.io/assets/caminandes-llamigos.jpg"
		>
			<source src="https://sveltejs.github.io/assets/caminandes-llamigos.mp4" type="video/mp4" />
			Your browser does not support the video tag.
			<track kind="captions" /></video
		>
	</Fullscreen>
</div>

<style lang="scss">
	.content {
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		background: black;
	}

	video {
		z-index: 0;
		width: 100%;
		height: auto;
	}

	.overlay {
		position: absolute;
		width: 100%;
		height: 100%;
	}

	.skip {
		position: absolute;
		z-index: 1;
		width: 50%;
		height: 100%;
		top: 0;
		&.left {
			left: 0;
		}
		&.right {
			left: 50%;
		}
	}
	.playpause {
		z-index: 2;
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
		color: white;
		&.hidden {
			color: transparent;
		}
	}
	.skipb {
		z-index: 2;
		position: absolute;
		top: 50%;
		left: 30%;
		color: white;
		transform: translate(-50%, -50%);
	}

	.skipf {
		z-index: 2;
		position: absolute;
		top: 50%;
		left: 70%;
		color: white;
		transform: translate(-50%, -50%);
	}
	.hover {
		z-index: 2;
		position: absolute;
		bottom: 0;
		left: 0;
		width: calc(100% - 2% * 2);
		margin: 2%;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 5px;
	}
	time {
		color: white;
	}
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

