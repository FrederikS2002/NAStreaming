<script lang="ts">
	import { url } from '../../urls';

	import { setIcon, setThumb, setTrailer } from '../List/stores/baseData';

	import { uploadIcon, uploadThumb, uploadTrailer } from './upload';

	export let classs: string;
	export let uuid: string;
	let fileElement: undefined | HTMLInputElement;
	let file: undefined | FileList;
	const openFilePicker = (_: any) => {
		if (fileElement) {
			fileElement.click();
		}
	};
	$: if (file) {
		if (!file[0].type.includes('image/')) {
			console.log('invalid');
			file = undefined;
		} else {
			//upload
			switch (classs) {
				case 'upload_thumb_btn':
					uploadThumb(file[0]).then((_) => setThumb(url + '/thumbs/' + uuid + '.jpeg'));
					break;
				case 'upload_icon_btn':
					uploadIcon(file[0]).then((_) => setIcon(url + '/icons/' + uuid + '.jpeg'));
					break;
				case 'add_trailer':
					uploadTrailer(file[0]).then((_) => setTrailer(url + '/trailers/' + uuid + '.mp4'));
					break;
				default:
					console.error('Undifined Upload');
			}
			console.log('uplaoded');
			file = undefined;
		}
	}
</script>

<div class={classs} on:click={openFilePicker}>
	<input bind:this={fileElement} bind:files={file} type="file" accept="image/*" hidden />
	<svg
		stroke="currentColor"
		fill="none"
		stroke-width="2"
		viewBox="0 1 24 24"
		stroke-linecap="round"
		stroke-linejoin="round"
		xmlns="http://www.w3.org/2000/svg"
		><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="17 8 12 3 7 8" /><line
			x1="12"
			y1="3"
			x2="12"
			y2="15"
		/></svg
	>
</div>

<style lang="scss">
	@import 'upload.scss';
</style>
