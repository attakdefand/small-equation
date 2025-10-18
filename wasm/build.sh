#!/bin/bash

# Build script for WebAssembly version of the Trillion Dollar Equation project

# Install wasm-pack if not already installed
if ! command -v wasm-pack &> /dev/null
then
    echo "Installing wasm-pack..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WASM package with our custom Cargo.toml
echo "Building WASM package..."
wasm-pack build --target web --out-dir pkg --manifest-path Cargo-wasm.toml

echo "WASM build complete! Files are in the pkg/ directory."