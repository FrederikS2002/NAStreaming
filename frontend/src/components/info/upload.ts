import axios from 'axios';
import { addNew } from './List/stores/episodeOrder';
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
export const onDropFile = (e) => {
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
			// time: '0',
			// 	file: uplaodFiles[i]
		}
	}
};
const startUpload = async () => {
	for (let i = 0; i < array.length; i++) {
		if (array[i].epi == null && array[i].file) {
			await uploadMovie(movie, array[i].file, i + 1);
		}
	}
};
const uploadMovie = async (movie: string, file: File, epi: number) => {
	let data = new FormData();
	data.append('movie', movie);
	data.append('epi', epi.toString());
	data.append('file', file);
	const options = {
		onUploadProgress: (progressEvent) => {
			const { loaded, total } = progressEvent;
			let percent = Math.floor((loaded * 100) / total);
			changeTime(percent.toString() + '/100', epi - 1);
		}
	};
	let res = await axios.post(`${url}/upload_episodes`, data, options);
	return res;
};
