#!/bin/bash

# Mustache JSON5 Formatter Installation Script for Zed Extension
# This script installs the mustache-json5-fmt formatter binary

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FORMATTER_DIR="$SCRIPT_DIR/../../formatter"
INSTALL_DIR="${HOME}/.local/bin"

echo "🔧 Installing Mustache JSON5 Formatter..."

# Check if Rust/Cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is required but not installed."
    echo "   Please install Rust from: https://rustup.rs/"
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Build the formatter
echo "📦 Building formatter..."
cd "$FORMATTER_DIR"
cargo build --release

# Copy the binary to install directory
echo "📂 Installing to $INSTALL_DIR..."
cp target/release/mustache-json5-fmt "$INSTALL_DIR/"

# Make sure it's executable
chmod +x "$INSTALL_DIR/mustache-json5-fmt"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo "⚠️  Warning: $INSTALL_DIR is not in your PATH"
    echo "   Add this line to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "   export PATH=\"$INSTALL_DIR:\$PATH\""
    echo ""
fi

# Test the installation
if command -v mustache-json5-fmt &> /dev/null; then
    VERSION=$(mustache-json5-fmt --version)
    echo "✅ Successfully installed: $VERSION"
    echo "🎉 Formatter is ready to use with Zed!"
else
    echo "⚠️  Installation completed, but formatter not found in PATH"
    echo "   You may need to restart your terminal or update your PATH"
fi

echo ""
echo "Usage examples:"
echo "  mustache-json5-fmt file.mustache_json5"
echo "  mustache-json5-fmt --stdin < file.mustache_json5"
echo "  mustache-json5-fmt --write --indent-size 4 file.mustache_json5"
