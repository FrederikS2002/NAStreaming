<script lang="ts">
    const redirectToNext = () => {
        //TODO REDIRECT TO NEXT
    }
</script>
<div on:click={redirectToNext}>
    <h1>Next</h1>
</div>
<style lang="scss">
  div {
    position: absolute;
    z-index: 2;
    bottom: 10%;
    right: 0;
    padding: 2% 5%;
    color: white;
    background-color: rgba(80, 80, 80, 0.6);
    border-radius: 5px 0 0 5px;
  }
</style>