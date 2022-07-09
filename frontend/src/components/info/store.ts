import { writable } from 'svelte/store';

const { subscribe, set, update } = writable<boolean>();

const setEditMode = (data: boolean) => set(data);

const edit_mode_sub = subscribe;
export { edit_mode_sub, setEditMode };
