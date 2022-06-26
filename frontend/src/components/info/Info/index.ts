import { setScrollProgress } from '../List/stores/drag';

const onScroll = (e: any) => {
	setScrollProgress(e.target.scrollTop);
};

const generateOrder = (epilist: any[]) => {
	let result: any[] = [];
	for (let i = 0; i < epilist.length; i++) {
		result.push({
			epi: epilist[i].epi,
			new: epilist[i].epi,
			type: 'epi'
		});
	}
	return result.sort((a, b) => a.new - b.new);
};
export { onScroll, generateOrder };
