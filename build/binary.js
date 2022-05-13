const { Binary } = require('binary-install');
const os = require('os');
const cTable = require('console.table');
const package = require('../package.json');




const REPO_URL = 'https://github.com/adamgian/minify-selectors';
const SUPPORTED_PLATFORMS = [
	{
		type: 'Windows_NT',
		architecture: 'x64',
		binary: 'windows-64.exe',
	},
	{
		type: 'Windows_NT',
		architecture: 'x32',
		binary: 'windows-32.exe',
	},
	{
		type: 'Linux',
		architecture: 'x64',
		binary: 'linux-64'
	},
	{
		type: 'Linux',
		architecture: 'arm64',
		binary: 'linux-arm64',
	},
	{
		type: 'Linux',
		architecture: 'x32',
		binary: 'linux-32',
	},
	{
		type: 'Darwin',
		architecture: 'x64',
		binary: 'macos-64',
	},
	{
		type: 'Darwin',
		architecture: 'arm64',
		binary: 'macos-arm64',
	},
];

const error = (message) => {
	console.error(message);
	process.exit(1);
};

const getPlatform = () => {
	const arch = os.arch();
	const type = os.type();

	for(let platform of SUPPORTED_PLATFORMS) {
		if (
			arch === platform.architecture
			&& type === platform.type
		) return platform;
	}

	error(
		`Unsupported platform: ${ type } ${ arch }.\nCurrently supported platforms are:\n\n${ cTable.getTable(SUPPORTED_PLATFORMS) }\n\nPlease feel free to notify us by creating an issue here: ${ package.bugs.url }.`
	);
};

const getBinary = () => {
	const platform = getPlatform();
	const url = `${ REPO_URL }/releases/download/v${ package.version }/${ package.name }-${ platform.binary }.tar.gz`;
	return new Binary(package.name, url);
};

const run = () => {
	const binary = getBinary();
	binary.run();
};

const install = () => {
	const binary = getBinary();
	binary.install();
};

module.exports = {
	install,
	run
};
