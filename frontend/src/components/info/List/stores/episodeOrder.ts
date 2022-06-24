import { writable } from 'svelte/store';

export type test = {
	epi: number | null;
	new: number;
	type: string;
};

const { subscribe, set, update } = writable<test[]>();

const addNew = (input: any) => {
	update((n: test[]) => {
		if(!input.new) {
			if(n.length > 0) {
			input.new = n[n.length - 1].new + 1;
			}else{
				input.new = 1;
			}
		}
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
		let newlist;
		// update list
		if (startIndex < dropIndex) {
			newlist = [
				...list.slice(0, dropIndex - 1),
				dragItem,
				...list.slice(dropIndex - 1, list.length)
			];
		} else {
			newlist = [...list.slice(0, dropIndex), dragItem, ...list.slice(dropIndex, list.length)];
		}
		for (let i = 0; i < newlist.length; i++) {
			newlist[i].new = i + 1;
		}
		return [...newlist];
	});
};

const setEpisodeOrderArr = (value: test[]) => {
	set(value);
};
const epi_order_subscribe = subscribe;
export { addNew, epi_order_subscribe, changeOrder, setEpisodeOrderArr };
