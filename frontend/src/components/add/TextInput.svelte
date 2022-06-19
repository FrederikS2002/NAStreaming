<script lang="ts">
	export let placeholder: any;
	export let value = '';
	export let number = false;
</script>

<div>
	{#if number}
		<input {placeholder} type="number" bind:value />
	{:else}
		<input {placeholder} type="text" bind:value />
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
		margin: 0.2% 0%;
		box-sizing: border-box;
		border: none;
		border-bottom: 1px solid rgba(243, 0, 0, 0);
		background-color: rgba(0, 0, 0, 0.2);
		color: white;
		&:focus {
			outline: none;
			border-bottom: 1px solid rgba(243, 0, 0, 0.5);
		}
	}
</style>
