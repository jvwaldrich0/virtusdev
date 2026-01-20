#!/bin/bash
set -e

source "$HOME/.cargo/env"

echo "Building VirtusDev..."
cargo build --release

echo ""
echo "✅ Build complete!"
echo ""
echo "Run with: ./run.sh"
