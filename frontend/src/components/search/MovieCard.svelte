<script lang="ts">
	export let movie;
</script>

<a sveltekit:noscroll sveltekit:prefetch href={"/info/"+movie.uuid}><div class="card">
	<img src={"http://127.0.0.1:8080/covers/"+movie.uuid+".jpeg"} alt="Image" />
	<div class="title">
		<h1>{movie.titles}</h1>
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
