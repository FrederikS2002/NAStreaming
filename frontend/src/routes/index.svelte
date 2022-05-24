<script lang="ts">
    import SearchMovieMenu from "../components/search/SearchMovieMenu.svelte";
    let json = [
		{
			title: 'Test',
			uuid: 'test',
			image_src:
				'https://www.komar.de/media/catalog/product/cache/4/image/9df78eab33525d08d6e5fb8d27136e95/4/-/4-4113_star_wars_movie_poster_rey_ma.jpg'
		}
	];
    for(let i = 0; i < 50; i++){
        json.push(json[0]);
    } 
</script>
<SearchMovieMenu json={json} />

