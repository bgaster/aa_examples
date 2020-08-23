#!/bin/bash

if [ $# -eq 0 ]; then
    echo "No arguments provided"
    exit 1
fi

cargo build --release --target wasm32-unknown-unknown

if [ $? -eq 0 ]
then
    echo Optimizing WASM output into pkg/$1.wasm

    if [ ! -d "./pkg" ] 
    then 
        mkdir ./pkg
    fi

    wasm-opt -O4 --enable-simd target/wasm32-unknown-unknown/release/$1.wasm -o pkg/$1.wasm
else 
    echo Compile failed
fi