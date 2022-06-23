import { writable } from 'svelte/store';

export type test2 = {
	uuid: string;
	epi: number;
	title: string;
	description: string;
	thumb: string;
	progress: number;
};

const { subscribe, set, update } = writable<test2[]>();

const addNew = (input: test2) => {
	update((n: test2[]) => {
		return [...n, input];
	});
};

const setArr = (value: test2[]) => {
	set(value);
};
const epi_sub = subscribe;
export { addNew, epi_sub, setArr };
