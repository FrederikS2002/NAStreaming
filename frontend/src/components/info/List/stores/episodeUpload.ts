import { writable } from 'svelte/store';
import type { episodeUpload } from '../../types';

const { subscribe, set, update } = writable<episodeUpload[]>();

const addNewUpload = (input: any) => {
	update((n: episodeUpload[]) => {
		input.progress = 0;
		if (!input.index) {
			if (n && n.length > 0) {
				input.index = n[n.length - 1].index + 1;
			} else {
				input.index = 0;
				return [input];
			}
		}
		return [...n, input];
	});
};

const epi_data_subscribe = subscribe;
export { addNewUpload, epi_data_subscribe };
