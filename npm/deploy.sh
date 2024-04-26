#!/bin/bash

jq -c '.[]' npm/platforms.json | while read build; do
	package_name=$(jq -r '.name' <<< "$build")
	platform_label=$(jq -r '.platform' <<< "$build")
	node_platform=$(jq -r '.nodePlatform' <<< "$build")
	architecture=$(jq -r '.architecture' <<< "$build")
	rust_target=$(jq -r '.rustTarget' <<< "$build")

	mkdir -p "npm/$package_name"
	cp "npm/package.json" "npm/$package_name/package.json"

	sed -i "s/FIXME_VERSION/$1/g" "npm/$package_name/package.json"
	sed -i "s/FIXME_BINARY/$package_name/g" "npm/$package_name/package.json"
	sed -i "s/FIXME_PLATFORM/$platform_label/g" "npm/$package_name/package.json"
	sed -i "s/FIXME_NODE_PLATFORM/$node_platform/g" "npm/$package_name/package.json"
	sed -i "s/FIXME_ARCHITECTURE/$architecture/g" "npm/$package_name/package.json"

	rust_target_path="merged-artifacts/$rust_target/release/"
	rust_target_name="minify-selectors"
	if [ "$node_platform" == "win32" ]; then
		rust_target_name+=".exe"
	fi

	mkdir -p "npm/$package_name/bin"
	cp "$rust_target_path$rust_target_name" "npm/$package_name/bin/$rust_target_name"
	if [ "$node_platform" != "win32" ]; then
		chmod +x "npm/$package_name/bin/$rust_target_name"
	fi

	sed -i "s/FIXME_EXECUTABLE/$rust_target_name/g" "npm/$package_name/package.json"
done
