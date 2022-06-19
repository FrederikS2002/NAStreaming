<script lang="ts">
	import { changeOrder } from './stores/episodes_store';

	export let epi;
	import { subscribedrag, updateDragOverIndex } from './stores/localstore';

	let item;
	let dragIndexes;
	subscribedrag((value) => (dragIndexes = value));

	const onDragEnter = () => {
		if (dragIndexes.dragStartIndex == null) return;
		if (item) {
			item.classList.add('dragover');
		}
		if (dragIndexes.dragOverIndex == null || dragIndexes.dragOverIndex != epi) {
			updateDragOverIndex(epi);
			let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
			if (ghostNode) {
				if (epi == dragIndexes.dragStartIndex) {
					ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + '. ';
				} else if (dragIndexes.dragStartIndex && dragIndexes.dragStartIndex > epi) {
					ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + '. ';
				} else {
					ghostNode.childNodes[0].childNodes[0].textContent = epi + '. ';
				}
			}
		}
	};

	const onDragLeave = () => {
		if (dragIndexes.dragStartIndex == null) return;
		if (item) {
			item.classList.remove('dragover');
		}
	};

	const onDrop = () => {
		if (dragIndexes.dragStartIndex == null) return;
		if (item) {
			item.classList.remove('dragover');
		}
		changeOrder(dragIndexes.dragStartIndex, epi);
	};

	const onDragOver = () => {
		if (dragIndexes.dragStartIndex == null) return;
		if (item && item.classList.contains('dragover')) {
			item.classList.add('dragover');
		}
	};
</script>

<div
	class="item"
	bind:this={item}
	draggable={true}
	on:drop={onDrop}
	on:dragover={onDragOver}
	on:dragenter={onDragEnter}
	on:dragleave={onDragLeave}
/>

<style lang="scss">
	.item {
		display: flex;
		width: 100%;
		padding: 2%;
		background: rgba(0, 0, 0, 0);
		border-radius: 5px;
		height: 5vh;
	}
</style>
