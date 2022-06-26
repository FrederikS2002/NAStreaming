import { changeOrder } from '../stores/episodeOrder';
import {
	updateDragStartIndex,
	removeMouseHeight,
	updateMouseHeight,
	removeMouseHeightStart,
	removeDragStartIndex,
	updateTurnPoints,
	subscribedrag
} from '../stores/store';

export const onMouseDown = (epi: number) => {
	updateDragStartIndex(epi);
	updateTurnPoints(epi, null);
};

export const onMouseUp = () => {
	let test: any;
	subscribedrag(v => test = v);
	if(!test.dragStartIndex) return;
	changeOrder(test.dragStartIndex - 1, test.dragOverIndex);
	//console.log(test.dragStartIndex, test.dragOverIndex);
	removeMouseHeight();
	removeMouseHeightStart();
	removeDragStartIndex();
};

export const onDrag = (e: any) => {
	updateMouseHeight(e.target.offsetTop);
};

export const onDragEnd = () => {
	removeMouseHeightStart();
	removeMouseHeight();
};
