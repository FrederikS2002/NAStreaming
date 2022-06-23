import { writable } from 'svelte/store';

export type test = {
	epi: number | null;
	new: number;
	type: string;
};

const { subscribe, set, update } = writable<test[]>();

const addNew = (input: test) => {
	update((n: test[]) => {
		return [...n, input];
	});
};

const changeOrder = (startIndex: number, dropIndex: number) => {
	update((n: test[]) => {
		// get draged item
		const dragItem = n[startIndex];

		// delete draged item in list
		const list = [...n];
		list.splice(startIndex, 1);

		// update list
		if (startIndex < dropIndex) {
			return [...list.slice(0, dropIndex - 1), dragItem, ...list.slice(dropIndex - 1, list.length)];
		} else {
			return [...list.slice(0, dropIndex), dragItem, ...list.slice(dropIndex, list.length)];
		}
	});
};
const epi_order_subscribe = subscribe;
export { addNew, epi_order_subscribe, changeOrder };
