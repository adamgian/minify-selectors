#!/bin/bash

jq -c '.[]' npm/platforms.json | while read build; do
	package_name="minify-selectors"
	platform_name=$(jq -r '.name' <<< "$build")
	platform_label=$(jq -r '.platform' <<< "$build")
	node_platform=$(jq -r '.nodePlatform' <<< "$build")
	architecture=$(jq -r '.architecture' <<< "$build")
	rust_target=$(jq -r '.rustTarget' <<< "$build")
	rust_target_path="merged-artifacts/$rust_target/release/"

	if [ "$node_platform" == "win32" ]; then
		package_name+=".exe"
	fi

	mkdir -p "npm/$platform_name"

	# Copy binary into build package directory
	mkdir -p "npm/$platform_name/bin"
	cp "$rust_target_path$package_name" "npm/$platform_name/bin/$package_name"
	if [ "$node_platform" != "win32" ]; then
		chmod +x "npm/$platform_name/bin/$package_name"
	fi

	# Create build package.json
	cp "npm/package.binaries.json" "npm/$platform_name/package.json"
	sed -i "s/FIXME_VERSION/$1/g" "npm/$platform_name/package.json"
	sed -i "s/FIXME_BINARY/$platform_name/g" "npm/$platform_name/package.json"
	sed -i "s/FIXME_PLATFORM/$platform_label/g" "npm/$platform_name/package.json"
	sed -i "s/FIXME_NODE_PLATFORM/$node_platform/g" "npm/$platform_name/package.json"
	sed -i "s/FIXME_ARCHITECTURE/$architecture/g" "npm/$platform_name/package.json"
	sed -i "s/FIXME_EXECUTABLE/$package_name/g" "npm/$platform_name/package.json"

	# Create build README.md
	cp "npm/README.md" "npm/$platform_name/README.md"
	sed -i "s/FIXME_RUST_TARGET/$rust_target/g" "npm/$platform_name/README.md"
done
