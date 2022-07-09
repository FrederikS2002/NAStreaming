import { create_movie, url } from '../urls';
type temp = { success: boolean; data: any };
export async function createMovie(
	titles: string[] | undefined,
	categories: string[] | undefined,
	type_: { value: string; label: string } | null,
	age_restriction: string | undefined,
	cover: FileList | undefined
): Promise<temp> {
	const data = new FormData();
	if (titles) data.append('titles', mkRd(titles));
	if (categories) data.append('categories', mkRd(categories));
	if (type_) data.append('type', type_.value);
	if (age_restriction) data.append('age_restriction', saveNum(age_restriction));
	if (cover && cover[0]) data.append('cover', cover[0], 'test.jpeg');
	try {
		const req = await fetch(url + create_movie, {
			method: 'post',
			//make sure to serialize your JSON body
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

const mkRd = (stringy: string[]) =>
	',' + stringy.map((s) => s.replaceAll(/,/g, '&comma;')).join(',') + ',';
const saveNum = (num: string) => num; //num.replace('[^d]', '');
