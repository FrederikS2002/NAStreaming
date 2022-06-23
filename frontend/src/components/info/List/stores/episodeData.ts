import { writable } from 'svelte/store';

type test = {
	uuid: string;
	epi: number;
	title: string;
	description: string;
	thumb: string;
	progress: number;
};

const { subscribe, set, update } = writable<test[]>();

const addNew = (input: test) => {
	update((n: test[]) => {
		return [...n, input];
	});
};

const setArr = (value: test[]) => {
	set(value);
};
const epi_sub = subscribe;
export { addNew, epi_sub, setArr };
