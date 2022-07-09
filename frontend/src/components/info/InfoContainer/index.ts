import { onMouseUp } from '../List/Episode/edit';

export const onMouseUpL = (_: any) => {
	let dragged = document.querySelector('.dragged');
	if (dragged) {
		dragged.classList.remove('dragged');
	}
	onMouseUp();
};
