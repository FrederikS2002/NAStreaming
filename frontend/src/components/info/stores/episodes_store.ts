import { writable } from 'svelte/store';

type test = {
	uuid: string;
	title: string;
	epi: number;
	time: string;
	img_src: string;
	file: null | File;
};

const { subscribe, set, update } = writable<test[]>([]);

const addNew = (input: test) => {
	update((n: test[]) => {
		return [...n, input];
	});
};
const changeTime = (time: string, index: number) => {
	update((n: test[]) => {
		n[index].time = time;
		return [...n];
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
const epi_sub = subscribe;
export { changeTime, addNew, epi_sub, changeOrder };
