const getTime = (time: number) => {
	if (isNaN(time)) {
		time = 0;
	}
	const date = new Date(0);
	date.setSeconds(Math.round(time)); // specify value for SECONDS here
	const h = date.getHours() - 1;
	const m = date.getMinutes();
	const s = date.getSeconds();

	// add a zero in front of numbers<10
	const mn = checkTime(m);
	const sn = checkTime(s);
	let result = '';
	if (h > 0) {
		result += checkTime(h) + ':';
	}
	result += mn + ':' + sn;
	return result;
};

const checkTime = (value: number) => {
	if (value < 10) {
		return '0' + value;
	}
	return value;
};

export default getTime;
