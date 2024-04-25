#!/bin/bash

jq -c '.[]' platforms.json | while read build; do
	platform_label=$(jq -r '.platform' <<< "$build")
	node_platform=$(jq -r '.nodePlatform' <<< "$build")
	architecture=$(jq -r '.architecture' <<< "$build")
	binary_label=$(jq -r '.binary' <<< "$build")

	mkdir $binary_label
	cp package.json "./$binary_label/package.json"
done
