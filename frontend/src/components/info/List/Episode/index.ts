import { url, play } from "../../../urls";
import { onDestroy } from "svelte";
import { epi_sub, type test2 } from "../stores/episodeData";

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
	let uuid: string = '';
	onDestroy(epi_sub((v) => (uuid = v.filter((v) => v.epi === epi)[0].uuid)));
	console.log(uuid);
	return url + play + uuid;
};

const oorb = (top: number, height: number, pos: number, active: boolean) => {
	if (active) {
		if (pos < top + height / 2) {
			return `margin-top: ${height}px`;
		} else {
			return `margin-bottom: ${height}px`;
		}
	}
	return '';
};

const getEpisodeData = (epi: number | null) => {
	if(epi == null) return {title: "", description: "", thumb:""};
	let res: test2 | null = null;
	onDestroy(epi_sub((v) => (res = v.filter((v) => v.epi === epi)[0])));
	return res;
}

export { generateStyle, generateFML, generateUrl, oorb, getEpisodeData };
