#!/bin/bash

jq -c '.[]' platforms.json | while read build; do
	platform_label=$(jq -r '.platform' <<< "$build")
	node_platform=$(jq -r '.nodePlatform' <<< "$build")
	architecture=$(jq -r '.architecture' <<< "$build")
	binary_label=$(jq -r '.binary' <<< "$build")

	mkdir -p $binary_label
	cp package.json "./$binary_label/package.json"

	sed -i "s/FIXME_VERSION/$1/g" ./$binary_label/package.json
	sed -i "s/FIXME_BINARY/$binary_label/g" ./$binary_label/package.json
	sed -i "s/FIXME_PLATFORM/$platform_label/g" ./$binary_label/package.json
	sed -i "s/FIXME_NODE_PLATFORM/$node_platform/g" ./$binary_label/package.json
	sed -i "s/FIXME_ARCHITECTURE/$architecture/g" ./$binary_label/package.json

	echo "$binary_label @$1"
done
