function getBinary() {
	try {
		const getBinary = require('./get-binary');
		return getBinary();
	}
	catch (err) {}
}

const binary = getBinary();
if (binary) binary().uninstall();
