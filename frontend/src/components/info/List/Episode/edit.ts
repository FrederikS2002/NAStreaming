import {
	updateDragStartIndex,
	updateMouseHeightStart,
	removeMouseHeight,
	updateMouseHeight,
	removeMouseHeightStart,
	removeDragStartIndex,
	updateTurnPoints
} from '../stores/store';

export const onMouseDown = (epi: number) => {
	//delete default ghost
	// let image = new Image(0, 0);
	// image.src = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7';
	// e.dataTransfer.setDragImage(image, 0, 0);
	// e.dataTransfer.effectAllowed = 'move';

	updateDragStartIndex(epi);
	updateTurnPoints(epi, null)
};

export const onMouseUp = () => {
	removeMouseHeight();
	removeMouseHeightStart();
	removeDragStartIndex();
}

export const onDrag = (e: any) => {
	updateMouseHeight(e.target.offsetTop);
};

export const onDragEnd = () => {
	removeMouseHeightStart();
	removeMouseHeight();
};


// const onDragEnter = () => {
//     if (dragIndexes.dragStartIndex == null) return;
//     if (item) {
//         item.classList.add('dragover');
//     }
//     if (dragIndexes.dragOverIndex == null || dragIndexes.dragOverIndex != epi) {
//         updateDragOverIndex(epi);
//         let ghostNode: HTMLElement | null = document.querySelector('#ghostNode');
//         if (ghostNode) {
//             if (epi == dragIndexes.dragStartIndex) {
//                 ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + '. ';
//             } else if (dragIndexes.dragStartIndex && dragIndexes.dragStartIndex > epi) {
//                 ghostNode.childNodes[0].childNodes[0].textContent = epi + 1 + '. ';
//             } else {
//                 ghostNode.childNodes[0].childNodes[0].textContent = epi + '. ';
//             }
//         }
//     }
// };

// const onDragLeave = () => {
//     if (dragIndexes.dragStartIndex == null) return;
//     if (item) {
//         item.classList.remove('dragover');
//     }
// };

// const onDrop = () => {
//     if (dragIndexes.dragStartIndex == null) return;
//     if (item) {
//         item.classList.remove('dragover');
//     }
//     if (dragIndexes.dragStartIndex) {
//         changeOrder(dragIndexes.dragStartIndex, epi);
//     }
// };

// const onDragOver = () => {
//     if (dragIndexes.dragStartIndex == null) return;
//     if (item && item.classList.contains('dragover')) {
//         item.classList.add('dragover');
//     }
// };
