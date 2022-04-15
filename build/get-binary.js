const { Binary } = require('binary-install');
const os = require('os');


function getPlatform() {
	const arch = os.arch();
	const type = os.type();

	if (type === 'Windows_NT' && arch === 'x64') return 'windows-64';
	else if (type === 'Linux' && arch === 'x64') return 'linux-64';
	else if (type === 'Linux' && arch === 'arm64') return 'linux-arm64';
	else if (type === 'Linux' && arch === 'x32') return 'linux-32';
	else if (type === 'Darwin' && arch === 'x64') return 'macos-64';
	else throw new Error (`Unsupported platform: ${ type } ${ arch }`);
}

function getBinary() {
	const platform = getPlatform();
	const version = require('../package.json').version;
	const url = `https://github.com/adamgian/minify-selectors/releases/download/v${ version }/minify-selectors-${ platform }.tar.gz`
	const name = 'minify-selectors';
	return new Binary(url, { name });
}

module.exports = getBinary;
