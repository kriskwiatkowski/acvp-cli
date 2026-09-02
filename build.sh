#!/bin/bash
set -e

echo "Building acvp-cli..."

# Build the Rust project
cargo build --release

echo ""
echo "Build complete!"
echo "Binary location: target/release/acvp-cli"
echo ""
echo "To use the tool, you'll need the modulewrapper binary."
echo "Build it from the parent directory:"
echo "  cd .."
echo "  cmake --build build --target modulewrapper"
echo ""
echo "Example usage:"
echo "  ./target/release/acvp-cli --wrapper ../build/modulewrapper/modulewrapper --regcap"
