<script lang="ts">
	export let placeholder: string;
	export let value: string = '';
	export let number: boolean = false;
</script>

<div class="tag_input">
	{#if number}
		<input {placeholder} type="number" bind:value />
	{:else}
		<input {placeholder} type="text" bind:value />
	{/if}
</div>

<style lang="scss">
	@import 'inputtext.scss';
</style>
