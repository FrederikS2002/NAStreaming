import { writable } from 'svelte/store';
import type { baseData } from '../../types';

const { subscribe, set, update } = writable<baseData>();

const setData = (input: baseData) => {
	set(input);
};

const setIcon = (input: string) => {
	update((n) => {
		n.icon = input;
		return n;
	});
};
const setThumb = (input: string) => {
	update((n) => {
		n.thumb = input;
		return n;
	});
};

const setTrailer = (input: string) => {
	update((n) => {
		n.trailer = input;
		return n;
	});
};

const base_data_sub = subscribe;
export { setData, base_data_sub, setIcon, setThumb, setTrailer };
