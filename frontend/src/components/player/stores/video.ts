import { writable } from 'svelte/store';
type test = {
	video: null | HTMLVideoElement;
	duration: number;
	currentTime: number;
	paused: boolean;
};

const { subscribe, set, update } = writable<test>({
	video: null,
	duration: 0,
	currentTime: 0,
	paused: true
});

const bindVideoData = (
	video: HTMLVideoElement | null,
	duration: number,
	currentTime: number,
	paused: boolean
) => {
	update((n) => {
		n.video = video;
		n.duration = duration;
		n.currentTime = currentTime;
		n.paused = paused;
		return n;
	});
};

const videoVolumeUp = () => {
	update((n) => {
		if (!n.video) return n;
		if (n.video.volume + 0.05 <= 1) {
			n.video.volume += 0.05;
		} else {
			n.video.volume = 1;
		}
		return n;
	});
};

const videoVolumeDown = () => {
	update((n) => {
		if (!n.video) return n;
		if (n.video.volume - 0.05 >= 0) {
			n.video.volume -= 0.05;
		} else {
			n.video.volume = 0;
		}
		return n;
	});
};

const skipForward = () => {
	update((n) => {
		n.currentTime += 5;
		return n;
	});
};

const skipBackward = () => {
	update((n) => {
		n.currentTime -= 5;
		return n;
	});
};

const videoSubscribe = subscribe;
export { videoSubscribe, bindVideoData, skipBackward, skipForward, videoVolumeUp, videoVolumeDown };
