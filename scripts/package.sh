rm -rf ./pkg
mkdir -p ./pkg
cp ./root/* ./pkg

# each audio graph 
mkdir ./pkg/nuke
cp -r nuke/nuke.json ./pkg/nuke
cp -r nuke/target/wasm32-unknown-unknown/release/nuke.wasm ./pkg/nuke
cp -r nuke/ui/index.html ./pkg/nuke

mkdir ./pkg/gain
cp -r gain/gain.json ./pkg/gain
cp -r gain/target/wasm32-unknown-unknown/release/gain.wasm ./pkg/gain
cp -r gain/ui/gain.html ./pkg/gain

mkdir ./pkg/reflect
cp -r reflect/reflect.json ./pkg/reflect
cp -r reflect/target/wasm32-unknown-unknown/release/reflect.wasm ./pkg/reflect
cp -r reflect/ui/index.html ./pkg/reflect

mkdir ./pkg/default
cp -r default/default.json ./pkg/default
cp -r default/target/wasm32-unknown-unknown/release/default.wasm ./pkg/default
cp -r default/ui/index.html ./pkg/default

