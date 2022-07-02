import { create_movie, url } from '../urls';

export async function makeRequest(
	titles: string[] | undefined,
	categories: string[] | undefined,
	type_: { value: string; label: string } | null,
	age_restriction: string | undefined,
	cover: FileList | undefined
) {
	const data = new FormData();
	if (titles) data.append('titles', mkRd(titles));
	if (categories) data.append('categories', mkRd(categories));
	if (type_) data.append('type', type_.value);
	if (age_restriction) data.append('age_restriction', saveNum(age_restriction));
	if (cover && cover[0]) data.append('cover', cover[0], 'test.jpeg');
	fetch(url + create_movie, {
		method: 'post',
		//make sure to serialize your JSON body
		body: data
	})
		.then((res) => res.json())
		.then((data) => {
			return { success: true, data: data };
		})
		.catch((err) => {
			return { success: false, data: err };
		});
}

const mkRd = (stringy: string[]) =>
	',' + stringy.map((s) => s.replaceAll(/,/g, '&comma;')).join(',') + ',';
const saveNum = (num: string) => num.replaceAll('[^d]', '');
