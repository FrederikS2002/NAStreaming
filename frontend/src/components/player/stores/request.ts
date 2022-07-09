import { writable } from 'svelte/store';
import type { EpisodeData } from '../types';

const { subscribe, set, update } = writable<EpisodeData>();

const setVideoData = (data: EpisodeData) => set(data);

const epi_data_sub = subscribe;
export { epi_data_sub, setVideoData };
