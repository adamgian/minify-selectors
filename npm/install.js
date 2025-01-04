// Adapted from esbuild's fantastic tried-and-tested install script

"use strict";
var child_process = require("child_process");
var fs = require("fs");
var https = require("https");
var os = require("os");
var path = require("path");
var zlib = require("zlib");



var MINIFY_SELECTORS_BINARY_PATH = process.env.MINIFY_SELECTORS_BINARY_PATH || MINIFY_SELECTORS_BINARY_PATH;
var isValidBinaryPath = (x) => !!x && x !== "/usr/bin/minify-selectors";

var availableBinaries = {
	"win32, x64": "@minify-selectors/windows-64",
	// TODO: "win32, arm64": "@minify-selectors/windows-arm64",
	"win32, ia32": "@minify-selectors/windows-32",
	"darwin, arm64": "@minify-selectors/darwin-64",
	"darwin, arm64": "@minify-selectors/darwin-arm64",
	"linux, x64": "@minify-selectors/linux-64",
	"linux, arm64": "@minify-selectors/linux-arm64",
	"linux, ia32": "@minify-selectors/linux-32",
};

var versionFromPackageJSON = require(path.join(__dirname, "package.json")).version;
var toPath = path.join(__dirname, "bin", "minify-selectors");
var isToPathJS = true;




function getPlatformAndBinary() {
	let currentPlatform;
	let subpath;
	let platformKey = `${process.platform}, ${os.arch()}`;

	if (platformKey in availableBinaries) {
		currentPlatform = availableBinaries[platformKey];
		subpath = "bin/minify-selectors";
		if (process.platform === "win32") subpath += ".exe";
	} else {
		throw new Error(`Unsupported platform: ${platformKey}`);
	}

	return { currentPlatform, subpath }
}

function downloadedBinPath(platform, subpath) {
	const libraryDir = path.dirname(require.resolve("minify-selectors"));
	return path.join(libraryDir, `downloaded-${platform.replace("/", "-")}-${path.basename(subpath)}`);
}

function validateBinaryVersion(...command) {
	command.push("--version");
	let stdout;
	try {
		stdout = child_process.execFileSync(command.shift(), command, {
			// Without this, this install script strangely crashes with the error
			// "EACCES: permission denied, write" but only on Ubuntu Linux when node is
			// installed from the Snap Store. This is not a problem when you download
			// the official version of node. The problem appears to be that stderr
			// (i.e. file descriptor 2) isn't writable?
			//
			// More info:
			// - https://snapcraft.io/ (what the Snap Store is)
			// - https://nodejs.org/dist/ (download the official version of node)
			// - https://github.com/evanw/esbuild/issues/1711#issuecomment-1027554035
			stdio: "pipe"
		}).toString().trim().split(" ")[1];
	} catch (err) {
		throw err;
	}

	if (stdout !== versionFromPackageJSON) {
		throw new Error(`Expected ${JSON.stringify(versionFromPackageJSON)} but got ${JSON.stringify(stdout)}`);
	}
}

function isYarn() {
	const { npm_config_user_agent } = process.env;
	if (npm_config_user_agent) {
		return /\byarn\//.test(npm_config_user_agent);
	}

	return false;
}

function fetch(url) {
	return new Promise((resolve, reject) => {
		https.get(url, (res) => {
			if ((res.statusCode === 301 || res.statusCode === 302) && res.headers.location)
				return fetch(res.headers.location).then(resolve, reject);
			if (res.statusCode !== 200)
				return reject(new Error(`Server responded with ${res.statusCode}`));

			let chunks = [];
			res.on("data", (chunk) => chunks.push(chunk));
			res.on("end", () => resolve(Buffer.concat(chunks)));
		}).on("error", reject);
	});
}

function extractFileFromTarGzip(buffer, subpath) {
	try {
		buffer = zlib.unzipSync(buffer);
	} catch (err) {
		throw new Error(`Invalid gzip data in archive: ${err && err.message || err}`);
	}

	let str = (i, n) => String.fromCharCode(...buffer.subarray(i, i + n)).replace(/\0.*$/, "");
	let offset = 0;
	subpath = `package/${subpath}`;

	while (offset < buffer.length) {
		let name = str(offset, 100);
		let size = parseInt(str(offset + 124, 12), 8);
		offset += 512;

		if (!isNaN(size)) {
			if (name === subpath) return buffer.subarray(offset, offset + size);
			offset += size + 511 & ~511;
		}
	}

	throw new Error(`Could not find ${JSON.stringify(subpath)} in archive`);
}

function installUsingNPM(currentPlatform, subpath, binPath) {
	const env = { ...process.env, npm_config_global: void 0 };
	const libraryDir = path.dirname(require.resolve("minify-selectors"));
	const installDir = path.join(libraryDir, "npm-install");
	fs.mkdirSync(installDir);

	try {
		fs.writeFileSync(path.join(installDir, "package.json"), "{}");
		child_process.execSync(
			`npm install --loglevel=error --prefer-offline --no-audit --progress=false ${currentPlatform}@${versionFromPackageJSON}`,
			{ cwd: installDir, stdio: "pipe", env }
		);
		const installedBinPath = path.join(installDir, "node_modules", currentPlatform, subpath);
		fs.renameSync(installedBinPath, binPath);
	} finally {
		try {
			removeRecursive(installDir);
		} catch {
		}
	}
}

function removeRecursive(dir) {
	for (const entry of fs.readdirSync(dir)) {
		const entryPath = path.join(dir, entry);
		let stats;

		try {
			stats = fs.lstatSync(entryPath);
		} catch {
			continue;
		}

		if (stats.isDirectory()) removeRecursive(entryPath);
		else fs.unlinkSync(entryPath);
	}

	fs.rmdirSync(dir);
}

function applyManualBinaryPathOverride(overridePath) {
	const pathString = JSON.stringify(overridePath);
	fs.writeFileSync(toPath, `#!/usr/bin/env node
require('child_process').execFileSync(${pathString}, process.argv.slice(2), { stdio: 'inherit' });
`);
	const libMain = path.join(__dirname, "lib", "main.js");
	const code = fs.readFileSync(libMain, "utf8");

	fs.writeFileSync(libMain, `var MINIFY_SELECTORS_BINARY_PATH = ${pathString};
${code}`);
}

function maybeOptimizePackage(binPath) {
	if (os.platform() !== "win32" && !isYarn()) {
		const tempPath = path.join(__dirname, "bin-minify-selectors");

		try {
			fs.linkSync(binPath, tempPath);
			fs.renameSync(tempPath, toPath);
			isToPathJS = false;
			fs.unlinkSync(tempPath);
		} catch {
		}
	}
}

async function downloadDirectlyFromNPM(pkg, subpath, binPath) {
	const url = `https://registry.npmjs.org/${pkg}/-/${pkg.replace("@minify-selectors/", "")}-${versionFromPackageJSON}.tgz`;
	console.error(`[minify-selectors] Trying to download ${JSON.stringify(url)}`);
	try {
		fs.writeFileSync(binPath, extractFileFromTarGzip(await fetch(url), subpath));
		fs.chmodSync(binPath, 493);
	} catch (e) {
		console.error(`[minify-selectors] Failed to download ${JSON.stringify(url)}: ${e && e.message || e}`);
		throw e;
	}
}

async function checkAndPreparePackage() {
	if (isValidBinaryPath(MINIFY_SELECTORS_BINARY_PATH)) {
		if (!fs.existsSync(MINIFY_SELECTORS_BINARY_PATH)) {
			console.warn(`[minify-selectors] Ignoring bad configuration: MINIFY_SELECTORS_BINARY_PATH=${MINIFY_SELECTORS_BINARY_PATH}`);
		} else {
			applyManualBinaryPathOverride(MINIFY_SELECTORS_BINARY_PATH);
			return;
		}
	}

	const { currentPlatform, subpath } = getPlatformAndBinary();
	let binPath;

	try {
		binPath = require.resolve(`${currentPlatform}/${subpath}`);
	} catch (e) {
		console.error(`[minify-selectors] Failed to find package "${currentPlatform}" on the file system
This can happen if you use the "--no-optional" flag. The "optionalDependencies"
package.json feature is used by minify-selectors to install the correct binary executable
for your current platform. This install script will now attempt to work around
this. If that fails, you need to remove the "--no-optional" flag to use minify-selectors.
`);
		binPath = downloadedBinPath(currentPlatform, subpath);

		try {
			console.error(`[minify-selectors] Trying to install package "${currentPlatform}" using npm`);
			installUsingNPM(currentPlatform, subpath, binPath);
		} catch (e2) {
			console.error(`[minify-selectors] Failed to install package "${currentPlatform}" using npm: ${e2 && e2.message || e2}`);

			try {
				await downloadDirectlyFromNPM(currentPlatform, subpath, binPath);
			} catch (e3) {
				throw new Error(`Failed to install package "${currentPlatform}"`);
			}
		}
	}

	maybeOptimizePackage(binPath);
}

checkAndPreparePackage().then(() => {
	if (isToPathJS) {
		validateBinaryVersion(process.execPath, toPath);
	} else {
		validateBinaryVersion(toPath);
	}
});
