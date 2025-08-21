#!/bin/bash

# Setup script for Comment Divider Zed Extension

echo "🚀 Setting up Comment Divider extension for Zed..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust not found. Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source ~/.cargo/env
else
    echo "✅ Rust is already installed"
fi

# Add wasm32-wasi target
echo "📦 Adding wasm32-wasi target..."
rustup target add wasm32-wasip1

if [ $? -eq 0 ]; then
    echo "✅ wasm32-wasi target added successfully"
else
    echo "❌ Failed to add wasm32-wasi target"
    exit 1
fi

# Build the extension
echo "🔨 Building extension..."
cargo build --target wasm32-wasi --release

if [ $? -eq 0 ]; then
    echo "✅ Extension built successfully!"
    echo ""
    echo "📝 Next steps:"
    echo "1. Open Zed"
    echo "2. Go to Extensions panel"
    echo "3. Click 'Install Dev Extension'"
    echo "4. Select this directory: $(pwd)"
    echo ""
    echo "🎉 Your Comment Divider extension is ready to use!"
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi
