import { writable } from 'svelte/store';
import type { baseData } from '../../types';

const { subscribe, set, update } = writable<baseData>();

const setData = (input: baseData) => {
	set(input);
};

const base_data_sub = subscribe;
export { setData, base_data_sub };
