# Build script for WebAssembly version of the Trillion Dollar Equation project

# Check if wasm-pack is installed
if (!(Get-Command wasm-pack -ErrorAction SilentlyContinue)) {
    Write-Host "Installing wasm-pack..."
    # Download and install wasm-pack
    curl -L https://github.com/rustwasm/wasm-pack/releases/download/v0.12.1/wasm-pack-init.exe -o wasm-pack-init.exe
    .\wasm-pack-init.exe -y
    Remove-Item wasm-pack-init.exe
}

# Build the WASM package with our custom Cargo.toml
Write-Host "Building WASM package..."
wasm-pack build --target web --out-dir pkg --manifest-path Cargo-wasm.toml

Write-Host "WASM build complete! Files are in the pkg/ directory."