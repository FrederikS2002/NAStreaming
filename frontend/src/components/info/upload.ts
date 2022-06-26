import axios from 'axios';
import { onDestroy } from 'svelte';
import { url } from '../urls';
import { addNew, epi_order_subscribe } from './List/stores/episodeOrder';
import { addNewUpload, epi_data_subscribe } from './List/stores/episodeUpload';
// const orderChanges = async () => {
// 	let results = [];
// 	for (let i = 0; i < array.length; i++) {
// 		if (array[i].epi && i + 1 != array[i].epi) {
// 			results.push(array[i].uuid + ' ' + (i + 1));
// 		}
// 	}
// 	let data = new FormData();
// 	data.append('order', results.join(','));
// 	const res = await fetch(`${url}/upload_order`, {
// 		method: 'post',
// 		//make sure to serialize your JSON body
// 		body: data
// 	});
// 	return await res.json();
// };
export const onDropFile = (e:any) => {
	e.preventDefault();
	let uplaodFiles = e.dataTransfer.files;
	if (uplaodFiles.length) {
		for (let i = 0; i < uplaodFiles.length; i++) {
			addNewUpload({index: null, name:  uplaodFiles[i].name, description: "", file: uplaodFiles[i]});
			let data = [{index: 0}];
			epi_data_subscribe((n) => data = n);
			addNew({
				epi: data[data.length - 1].index,
				new: null,
				type: 'file',
			});
		}
	}
};
export const startUpload = async (movie: string) => {
	let epi_order:any;
	let epi_file: any;
	onDestroy(epi_order_subscribe((n) => epi_order = n));
	epi_data_subscribe((n) => epi_file = n);
	if(epi_order){
		for (let i = 0; i < epi_order.length; i++) {
			if (epi_order[i].type = 'file') {
				await uploadMovie(movie, epi_file[epi_order[i].epi].file, epi_order[i].new);
				//TODO: Upload description
			}
		}
	}
};
const uploadMovie = async (movie: string, file: File, epi: number) => {
	let data = new FormData();
	data.append('movie', movie);
	data.append('epi', epi.toString());
	data.append('file', file);
	const options = {
		onUploadProgress: (progressEvent: { loaded: any; total: any; }) => {
			const { loaded, total } = progressEvent;
			//TODO: Update percent
			let percent = Math.floor((loaded * 100) / total);
		}
	};
	let res = await axios.post(`${url}/upload_episodes`, data, options);
	return res;
};
