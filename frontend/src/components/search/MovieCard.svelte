<script lang="ts">
	export let movie;
</script>

<a sveltekit:noscroll sveltekit:prefetch href={"/info/"+movie.uuid}><div class="card">
	<img src={movie.image_src} alt="Image" />
	<div class="title">
		<h1>{movie.title}</h1>
	</div>
</div></a>

<style lang="scss">
	img {
		width: 100%;
        height: 35vh;
        object-fit: cover;
        border-radius: 1rem;
        margin-bottom: 1rem;
	}
</style>
