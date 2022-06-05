<script lang="ts">
import TextInput from "./TextInput.svelte";
let titles;
let categories;
let type_;
let age_restriction;
let cover;

export async function makeRequest() {
    let data = new FormData();
    if(titles) data.append('titles', titles);
    if(categories) data.append('categories', categories);
    if(type_) data.append('type', type_);
    if(age_restriction) data.append('age_restriction', age_restriction);
    if(cover && cover[0]) data.append('cover', cover[0], "test.jpeg");
    fetch("http://127.0.0.1:8080/create_movie", {
        method: "post",
        //make sure to serialize your JSON body
        body: data,
    }).then(res => res.json())
        .then(data => {
            console.log(data)
        }).catch(err => {
        console.error('Error: ', err);
    });
}
</script>


<div class="overlay" on:click|self>
    <div class="menu">
        <TextInput placeholder="Name" bind:value={titles} />
        <TextInput placeholder="Category" bind:value={categories}/>
        <TextInput placeholder="Type" bind:value={type_} />
        <TextInput number={true} placeholder="age restriction" bind:value={age_restriction} />
        <input type="file" bind:files={cover}>
        <button on:click={makeRequest}>Submit</button>
    </div>
</div>

<style lang="scss">
    .overlay {
      z-index: 1;
      position: fixed;
      background: transparent;
      width: 100%;
      height: 100%;
      top: 0;
      left: 0;
    }

    .menu{
      position:absolute;
      left: 10%;
      top:10%;
      width: 80%;
      background: #505050;
      border-radius: 10px;
      height:80%;
    }
</style>