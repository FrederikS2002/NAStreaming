import {
	updateDragStartIndex,
	removeMouseHeight,
	updateMouseHeight,
	removeMouseHeightStart,
	removeDragStartIndex,
	updateTurnPoints
} from '../stores/store';

export const onMouseDown = (epi: number) => {
	updateDragStartIndex(epi);
	updateTurnPoints(epi, null);
};

export const onMouseUp = () => {
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
