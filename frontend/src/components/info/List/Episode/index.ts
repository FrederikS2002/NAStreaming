const generateStyle = (fml: number) => {
	if (fml == 0) {
		return 'border-top-left-radius: 10px;border-top-right-radius: 10px;';
	} else if (fml == 2) {
		return 'border-bottom-left-radius: 10px;border-bottom-right-radius: 10px;';
	}
	return;
};

const generateFML = (index: number, length: number) => {
	if (index == 0) {
		return 0;
	} else if (index + 1 == length) {
		return 2;
	} else {
		return 1;
	}
};
export { generateStyle, generateFML };
