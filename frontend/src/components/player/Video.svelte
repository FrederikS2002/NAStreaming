<script lang="ts">
    import { bindVideoData, videoSubscribe } from './stores/video.ts';

    let currentTime = 0;
    let duration = 0;
    let paused = true;
    let video = null;

    //TODO: FIX UPDATE time -1h
    $: bindVideoData(video, duration, currentTime,paused);
    videoSubscribe(value => currentTime = value.currentTime);

</script>
<video
        bind:currentTime={currentTime}
        bind:duration
        bind:paused
        bind:this={video}
        poster="https://sveltejs.github.io/assets/caminandes-llamigos.jpg"
        preload="metadata"
>
    <source src="https://sveltejs.github.io/assets/caminandes-llamigos.mp4" type="video/mp4"/>
    Your browser does not support the video tag.
    <track kind="captions"/>
</video
>

<style lang="scss">
  video {
    z-index: 0;
    width: 100%;
    height: auto;
  }
</style>