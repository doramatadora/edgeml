#!/bin/bash
fastly compute build

BUNDLE_DIR=pkg
PROJECT=edgeml-objstore
PKDIR=$BUNDLE_DIR/$PROJECT

# Create a bundle directory
mkdir -p $BUNDLE_DIR

# Create a folder for our application
rm -rf $PKDIR
mkdir -p $PKDIR
mkdir -p $PKDIR/bin

# Copy over the required files
cp fastly.toml $PKDIR
cp Cargo.toml $PKDIR

# Optimise the wasm some more
# https://github.com/WebAssembly/binaryen#tools
wasm-opt bin/main.wasm -O -o $PKDIR/bin/main.wasm 

# Archive the directory
(cd $BUNDLE_DIR && tar -czf $PROJECT.tar.gz $PROJECT)

echo "Bundled C@E module to $PKDIR.tar.gz. Uploading WASM package to Fastly."

fastly compute deploy
