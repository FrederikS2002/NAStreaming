<script lang="ts">
	/*let time = 0;
	let next_update = 0;
	$: if (time >= next_update) {
		//TODO: PUSH Progress
		next_update = time + 5;
	}

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
    }*/

	import VideoPlayer from '../../components/player/VideoPlayer.svelte';
</script>

<!--<svelte:window on:keydown={handleKeydown}/>-->
<VideoPlayer />
