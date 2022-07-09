import { url, play } from '../../../urls';
import { onDestroy } from 'svelte';
import { epi_sub } from '../stores/episodeData';
import { updateDragOverIndex } from '../stores/drag';
import { epi_data_subscribe } from '../stores/episodeUpload';
import type { episodeData } from '../../types';
import { base_data_sub } from '../stores/baseData';

const generateStyle = (fml: number) => {
	if (fml == 0) {
		return 'border-top-left-radius: 10px;border-top-right-radius: 10px;';
	} else if (fml == 2) {
		return 'border-bottom-left-radius: 10px;border-bottom-right-radius: 10px;';
	}
	return;
};

const generateFML = (index: number, length: number) => {
	if (index == 0) {
		return 0;
	} else if (index + 1 == length) {
		return 2;
	} else {
		return 1;
	}
};

const generateUrl = (epi: number) => {
	let temp: any;
	base_data_sub((v) => (temp = v));
	let uuid: string = '';
	epi_sub((v) => (uuid = v.filter((v) => v.epi === epi)[0].uuid));
	return play + temp.uuid + '/' + uuid;
};

const oorb = (
	epi: number,
	top: number,
	height: number,
	pos: number,
	active: boolean,
	currentPos: number
) => {
	if (active) {
		if (pos < top + height / 2) {
			if (currentPos != epi - 1) {
				updateDragOverIndex(epi - 1);
			}
			return `margin-top: ${height}px`;
		} else {
			if (currentPos != epi) {
				updateDragOverIndex(epi);
			}
			return `margin-bottom: ${height}px`;
		}
	}
	return '';
};

const getEpisodeData = (epi: number | null, type: string) => {
	if (type == 'epi') {
		if (epi == null) return { title: '', description: '', thumb: '' };
		let res: episodeData | null = null;
		onDestroy(epi_sub((v) => (res = v.filter((v) => v.epi === epi)[0])));
		return res;
	} else if (type == 'file') {
		if (epi == null) return { title: '', description: '', thumb: '' };
		let res: any;
		onDestroy(epi_data_subscribe((v) => (res = v.filter((v) => v.index === epi)[0])));
		return { title: res.name, description: res.description, thumb: '' };
	}
	return { title: '', description: '', thumb: '' };
};

export { generateStyle, generateFML, generateUrl, oorb, getEpisodeData };
