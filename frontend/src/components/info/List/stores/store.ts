import { writable } from 'svelte/store';

export type test = {
	dragStartIndex: null | number;
	mouseHeight: number | null;
	mouseHeightStart: number | null;
	cords: (number | null)[];
	scrollprogress: number;
	dragOverIndex: null | number;
};

const { subscribe, set, update } = writable<test>({
	dragStartIndex: null,
	mouseHeight: null,
	cords: [],
	scrollprogress: 0,
	mouseHeightStart: null,
	dragOverIndex: null
});

const updateDragOverIndex = (index: number | null) => {
	update((n) => {
		n.dragOverIndex = index;
		return n;
	});
};

const updateDragStartIndex = (index: number) => {
	update((n) => {
		n.dragStartIndex = index;
		return n;
	});
};

const removeDragStartIndex = () => {
	update((n) => {
		n.dragStartIndex = null;
		return n;
	});
};

const updateMouseHeight = (height: number) => {
	update((n) => {
		n.mouseHeight = height;
		return n;
	});
};
const removeMouseHeight = () => {
	update((n) => {
		n.mouseHeight = null;
		return n;
	});
};
const updateMouseHeightStart = (height: number) => {
	update((n) => {
		n.mouseHeightStart = height;
		return n;
	});
};
const removeMouseHeightStart = () => {
	update((n) => {
		n.mouseHeightStart = null;
		return n;
	});
};

const updateTurnPoints = (index: number, value: number | null) => {
	update((n) => {
		let mod = n.cords;
		mod[index] = value;
		n.cords = [...mod];
		return n;
	});
};

const setScrollProgress = (progress: number) => {
	update((n) => {
		n.scrollprogress = progress;
		return n;
	});
};
const subscribedrag = subscribe;
export {
	updateTurnPoints,
	updateDragOverIndex,
	updateDragStartIndex,
	subscribedrag,
	updateMouseHeight,
	removeMouseHeight,
	removeMouseHeightStart,
	updateMouseHeightStart,
	removeDragStartIndex,
	setScrollProgress
};
