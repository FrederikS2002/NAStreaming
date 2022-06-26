export interface Epilist {
	uuid: string;
	epi: number;
	title: string;
	description: string;
	thumb: string;
}

export interface RootObj {
	uuid: string;
	titles: string[];
	icon: string;
	thumb: string;
	color: number[];
	description: string;
	epilist: Epilist[];
}

export interface baseData {
	uuid: string;
	titles: string[];
	icon: string;
	thumb: string;
	color: number[];
	description: string;
}

export interface episodeData {
	uuid: string;
	epi: number;
	title: string;
	description: string;
	thumb: string;
	progress: number;
}

export interface episodeOrder {
	epi: number | null;
	new: number;
	type: string;
}

export interface episodeUpload {
	index: number;
	name: number;
	description: string;
	progress: number;
	file: any;
}

export interface drag {
	dragStartIndex: null | number;
	mouseHeight: number | null;
	mouseHeightStart: number | null;
	cords: (number | null)[];
	scrollprogress: number;
	dragOverIndex: null | number;
}
