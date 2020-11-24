#!/bin/bash
fastly compute build --force

# Create a bundle directory
mkdir -p pkg

# Create a folder for our application
mkdir -p pkg/hackadora

# Copy over the required files
cp fastly.toml pkg/hackadora
cp Cargo.toml pkg/hackadora
mkdir -p pkg/hackadora/bin

wasm-opt target/wasm32-wasi/release/hackadora.wasm -O -o pkg/hackadora/bin/main.wasm 

# Tar the directory
(cd pkg && tar -czf hackadora.tar.gz hackadora)

echo "Bundled C@E module to bpkg/hackadora.tar.gz. Upload this file to Fastly."
