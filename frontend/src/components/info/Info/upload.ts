import { url } from '../../urls';

export const uploadThumb = async (file: FileList, uuid: string) => {
	return await upload(file, 'thumb', uuid);
};

export const uploadTrailer = async (file: FileList, uuid: string) => {
	return await upload(file, 'trailer', uuid);
};

export const uploadIcon = async (file: FileList, uuid: string) => {
	return await upload(file, 'icon', uuid);
};

export const uploadCover = async (file: FileList, uuid: string) => {
	return await upload(file, 'cover', uuid);
};

async function upload(file: FileList, path: string, uuid: string) {
	const data = new FormData();
	data.append("uuid", uuid);
	data.append(path, file[0]);
	try {
		const req = await fetch(url + '/upload_files', {
			method: 'post',
			body: data
		});
		const json = await req.json();
		if (json == '200') {
			return { success: true, data: data };
		} else {
			return { success: false, data: data };
		}
	} catch (e) {
		return { success: false, data: e };
	}
}
