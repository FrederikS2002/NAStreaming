<script lang="ts">
import TextInput from "./TextInput.svelte";
let name = "";
let category = "";
let type_ = "";
let age = "";
let img = "";
$: console.log(JSON.stringify({
    titles: name,
    categories: category,
    type_: type_,
    age_restriction: age,
    img_src: img,
}))

export async function makeRequest(name, category, type_, age, img) {
    let data = new FormData();
    data.append('name', name);
    data.append('titles', name);
    data.append('categories', category);
    data.append('type_', type_);
    data.append('age_restriction', age);
    data.append('img_src', img);
    fetch("http://192.168.178.5:8080/add_movie", {
        method: "post",
        //make sure to serialize your JSON body
        body: data,
    })
        .then( (response) => {
            //do something awesome that makes the world a better place
        });
}
</script>


<div class="overlay" on:click|self>
    <div class="menu">
        <TextInput placeholder="Name" bind:value={name} />
        <TextInput placeholder="Category" bind:value={category}/>
        <TextInput placeholder="Type" bind:value={type_} />
        <TextInput number={true} placeholder="age restriction" bind:value={age}/>
        <TextInput placeholder="image src" bind:value={img} />
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