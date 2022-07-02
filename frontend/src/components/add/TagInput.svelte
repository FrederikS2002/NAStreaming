<script lang="ts">
	export let placeholder: string;
	export let value: string[] = [];

	let input = '';

	const handleKeypress = (e: KeyboardEvent) => {
		if (e.key === 'Enter') {
			value.push(input);
			input = '';
			value = [...value];
		}
	};

	const removeValue = (index: number) => {
		value.splice(index, 1);
		value = [...value];
	};
</script>

<div class="tag_input">
	{#each value as item, index}
		<div>
			<div>
				{item}
				<div class="x" on:click={(_) => removeValue(index)}>x</div>
			</div>
		</div>
	{/each}
	<input {placeholder} type="text" on:keypress={handleKeypress} bind:value={input} />
</div>

<style lang="scss">
	@import 'inputtags.scss';
</style>
