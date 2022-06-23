<script lang="ts">
	import getTime from '../convertSecondsToTime';
	import { videoSubscribe } from '../stores/video';

	let duration: number,
		currentTime: number,
		time1: HTMLUnknownElement,
		time2: HTMLUnknownElement,
		hover: HTMLDivElement;
	//TODO:Setting, onDestroy
	let remainingTime = true;
	videoSubscribe((value) => {
		duration = value.duration;
		currentTime = value.currentTime;
	});
	$: calculatePixel = () => {
		if (hover && time1 && time2) {
			return hover.clientWidth - (time1.offsetWidth + time2.offsetWidth + 10) + 'px';
		}
		return 0;
	};
</script>

<div bind:this={hover}>
	<!--TODO:Make responsive -->
	<time bind:this={time1}>{getTime(currentTime)}</time>
	<input
		type="range"
		min="0"
		max={duration}
		step="0.001"
		style="width: {calculatePixel()}"
		value={currentTime}
	/>
	<!-- on:input={(e) => (time = e.target.value)}--->
	<time bind:this={time2}
		>{remainingTime ? '-' + getTime(duration - currentTime) : getTime(duration)}</time
	>
</div>

<style lang="scss">
	div {
		z-index: 2;
		position: absolute;
		bottom: 0;
		left: 0;
		width: calc(100% - 2% * 2);
		margin: 2%;
		background: rgba(255, 255, 255, 0.2);
		border-radius: 5px;
	}
	time {
		color: white;
	}
</style>
