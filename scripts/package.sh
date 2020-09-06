#!/bin/bash

# Package up AA example release
# Really we need to move to a more robust was of generating a package, but
# for now we assume that each of the examples to be added are already built,
# including the corresponding files:
#
#    module/module.json
#    module/target/wasm32-unknown-unknown/release/module.wasm
#    module/ui/index.html
#    module/any extras required for example

rm -rf ./pkg
mkdir -p ./pkg
cp -r ./root/* ./pkg
 
# each audio graph 
mkdir ./pkg/nuke
cp -r nuke/nuke.json ./pkg/nuke
cp -r nuke/target/wasm32-unknown-unknown/release/nuke.wasm ./pkg/nuke
cp -r nuke/ui/index.html ./pkg/nuke/nuke.html

mkdir ./pkg/gain
mkdir ./pkg/gain/js
cp -r gain/js/* ./pkg/gain/js
cp -r gain/gain.json ./pkg/gain
cp -r gain/target/wasm32-unknown-unknown/release/gain.wasm ./pkg/gain
cp -r gain/ui/gain.html ./pkg/gain/gain.html

mkdir ./pkg/reflect
cp -r reflect/reflect.json ./pkg/reflect
cp -r reflect/target/wasm32-unknown-unknown/release/reflect.wasm ./pkg/reflect
cp -r reflect/ui/index.html ./pkg/reflect/reflect.html

mkdir ./pkg/vl1
cp -r vl1/vl1.json ./pkg/vl1
cp -r vl1/target/wasm32-unknown-unknown/release/vl1.wasm ./pkg/vl1
cp -r vl1/ui/index.html ./pkg/vl1/vl1.html

mkdir ./pkg/default
cp -r default/default.json ./pkg/default
cp -r default/target/wasm32-unknown-unknown/release/default.wasm ./pkg/default
cp -r default/ui/default.html ./pkg/default/default.html

