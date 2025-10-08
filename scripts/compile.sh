#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_ROOT}"

echo "Building VirtUSDev..."
make clean
make all
make keyboard_writer

echo ""
echo "Build complete!"
echo "  virtual_keyboard - Device emulator"
echo "  virtusdev        - Barcode writer tool"
echo "  keyboard_writer  - Symlink to virtusdev (backward compatibility)"
