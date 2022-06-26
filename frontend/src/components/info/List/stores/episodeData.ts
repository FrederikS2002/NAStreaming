import { writable } from 'svelte/store';
import type { episodeData } from '../../types';

const { subscribe, set, update } = writable<episodeData[]>();

const addNew = (input: episodeData) => {
	update((n: episodeData[]) => {
		return [...n, input];
	});
};

const setEpisodeData = (value: episodeData[]) => {
	set(value);
};
const epi_sub = subscribe;
export { addNew, epi_sub, setEpisodeData };
