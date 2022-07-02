<script lang="ts">
	export let placeholder: any;
	export let value = '';
	export let number = false;
</script>

<div class="tag_input">
	{#if number}
		<input {placeholder} type="number" bind:value />
	{:else}
		<input {placeholder} type="text" bind:value />
	{/if}
</div>

<style lang="scss">
	@import 'inputs.scss';
</style>
