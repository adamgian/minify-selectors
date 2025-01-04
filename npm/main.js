#!/usr/bin/env node

// Adapted from esbuild's fantastic tried-and-tested node script




var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __copyProps = (to, from, except, desc) => {

	if (from && typeof from === "object" || typeof from === "function") {
		for (let key of __getOwnPropNames(from)) {
			if (!__hasOwnProp.call(to, key) && key !== except) {
				__defProp(
					to, key,
					{ get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable }
				);
			}
		}
	}

	return to;
};

var __toESM = (mod, isNodeMode, target) => (
	target = mod != null ? __create(__getProtoOf(mod)) : {},
	__copyProps(
		// If the importer is in node compatibility mode or this is not an ESM
		// file that has been converted to a CommonJS file using a Babel-
		// compatible transform (i.e. "__esModule" has not been set), then set
		// "default" to the CommonJS "module.exports" for node compatibility.
		isNodeMode || !mod || !mod.__esModule
			? __defProp(target, "default", { value: mod, enumerable: true })
			: target,
		mod
	)
);




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

function pkgForSomeOtherPlatform() {
	const libMainJS = require.resolve("minify-selectors");
	const nodeModulesDirectory = path.dirname(path.dirname(path.dirname(libMainJS)));

	if (path.basename(nodeModulesDirectory) === "node_modules") {
		for (const unixKey in knownUnixlikePackages) {
			try {
				const pkg = knownUnixlikePackages[unixKey];
				if (fs.existsSync(path.join(nodeModulesDirectory, pkg))) return pkg;
			} catch {
			}
		}

		for (const windowsKey in knownWindowsPackages) {
			try {
				const pkg = knownWindowsPackages[windowsKey];
				if (fs.existsSync(path.join(nodeModulesDirectory, pkg))) return pkg;
			} catch {
			}
		}
	}

	return null;
}

function downloadedBinPath(platform, subpath) {
	const libraryDir = path.dirname(require.resolve("minify-selectors"));
	return path.join(libraryDir, `downloaded-${platform.replace("/", "-")}-${path.basename(subpath)}`);
}

function generateBinPath() {
	if (isValidBinaryPath(MINIFY_SELECTORS_BINARY_PATH)) {
		if (!fs.existsSync(MINIFY_SELECTORS_BINARY_PATH)) {
			console.warn(`[minify-selectors] Ignoring bad configuration: MINIFY_SELECTORS_BINARY_PATH=${MINIFY_SELECTORS_BINARY_PATH}`);
		} else {
			return { binPath: MINIFY_SELECTORS_BINARY_PATH };
		}
	}

	const { pkg, subpath } = pkgAndSubpathForCurrentPlatform();
	let binPath2;

	try {
		binPath2 = require.resolve(`${pkg}/${subpath}`);
	} catch (e) {
		binPath2 = downloadedBinPath(pkg, subpath);
		if (!fs.existsSync(binPath2)) {
			try {
				require.resolve(pkg);
			} catch {
				const otherPkg = pkgForSomeOtherPlatform();

				if (otherPkg) {
					let suggestions = `
Specifically the "${otherPkg}" package is present but this platform
needs the "${pkg}" package instead. People often get into this
situation by installing minify-selectors on Windows or macOS and copying "node_modules"
into a Docker image that runs Linux, or by copying "node_modules" between
Windows and WSL environments.

If you are installing with npm, you can try not copying the "node_modules"
directory when you copy the files over, and running "npm ci" or "npm install"
on the destination platform after the copy. Or you could consider using yarn
instead of npm which has built-in support for installing a package on multiple
platforms simultaneously.

If you are installing with yarn, you can try listing both this platform and the
other platform in your ".yarnrc.yml" file using the "supportedArchitectures"
feature: https://yarnpkg.com/configuration/yarnrc/#supportedArchitectures
Keep in mind that this means multiple copies of minify-selectors will be present.
`;
					if (pkg === packageDarwin_x64 && otherPkg === packageDarwin_arm64 || pkg === packageDarwin_arm64 && otherPkg === packageDarwin_x64) {
						suggestions = `
Specifically the "${otherPkg}" package is present but this platform
needs the "${pkg}" package instead. People often get into this
situation by installing minify-selectors with npm running inside of Rosetta 2 and then
trying to use it with node running outside of Rosetta 2, or vice versa (Rosetta
2 is Apple's on-the-fly x86_64-to-arm64 translation service).

If you are installing with npm, you can try ensuring that both npm and node are
not running under Rosetta 2 and then reinstalling minify-selectors. This likely involves
changing how you installed npm and/or node. For example, installing node with
the universal installer here should work: https://nodejs.org/en/download/. Or
you could consider using yarn instead of npm which has built-in support for
installing a package on multiple platforms simultaneously.

If you are installing with yarn, you can try listing both "arm64" and "x64"
in your ".yarnrc.yml" file using the "supportedArchitectures" feature:
https://yarnpkg.com/configuration/yarnrc/#supportedArchitectures

Keep in mind that this means multiple copies of minify-selectors will be present.
`;
					}

					throw new Error(`
You installed minify-selectors for another platform than the one you're currently using.
This won't work because minify-selectors is written with native code and needs to
install a platform-specific binary executable.

${suggestions}
`);
				}
				throw new Error(`The package "${pkg}" could not be found, and is needed by minify-selectors.
If you are installing minify-selectors with npm, make sure that you don't specify the
"--no-optional" or "--omit=optional" flags. The "optionalDependencies" feature
of "package.json" is used by minify-selectors to install the correct binary executable
for your current platform.`);
			}
			throw e;
		}
	}

	if (/\.zip\//.test(binPath2)) {
		let pnpapi;

		try {
			pnpapi = require("pnpapi");
		} catch (e) {
		}
		if (pnpapi) {
			const root = pnpapi.getPackageInformation(pnpapi.topLevel).packageLocation;
			const binTargetPath = path.join(
				root,
				"node_modules",
				".cache",
				"minify-selectors",
				`pnpapi-${pkg.replace("/", "-")}-${"0.24.2"}-${path.basename(subpath)}`
			);

			if (!fs.existsSync(binTargetPath)) {
				fs.mkdirSync(path.dirname(binTargetPath), { recursive: true });
				fs.copyFileSync(binPath2, binTargetPath);
				fs.chmodSync(binTargetPath, 493);
			}

			return { binPath: binTargetPath };
		}
	}

	return { binPath: binPath2 };
}

var { binPath } = generateBinPath();

require("child_process").execFileSync(binPath, process.argv.slice(2), { stdio: "inherit" });
