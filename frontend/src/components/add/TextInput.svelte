<script>
    export let placeholder;
    export let value = "";
    export let number = false;
</script>
<div>
    {#if number}
        <input placeholder={placeholder} type="number" bind:value={value}  />
    {:else}
        <input placeholder={placeholder} type="text" bind:value={value}  />

    {/if}
</div>
<style lang="scss">
  div {
    margin: 2%;
  }
    input {
      width: 100%;
      height: 5vh;
      padding: 0 1%;
      margin: .2% 0%;
      box-sizing: border-box;
      border: none;
      border-bottom: 1px solid rgba(243, 0, 0, 0);
      background-color: rgba(0, 0, 0, .2);
      color: white;
      &:focus{
        outline: none;
        border-bottom: 1px solid rgba(243, 0, 0, 0.5);

      }

    }
</style>