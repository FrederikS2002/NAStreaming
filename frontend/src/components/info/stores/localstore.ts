import { writable } from 'svelte/store';

type test = {
    dragStartIndex: null|number,
    dragOverIndex: null|number,
};

const {subscribe, set, update} = writable<test>({
    dragStartIndex: null,
    dragOverIndex: null,
});

const updateDragOverIndex = (index:number) => {
    update((n) => {
        n.dragOverIndex = index;
        return n;
    });
};

const updateDragStartIndex = (index:number) => {
    update((n) => {
        n.dragStartIndex = index;
        return n;
    });
};
const subscribedrag = subscribe;
export {updateDragOverIndex, updateDragStartIndex, subscribedrag}