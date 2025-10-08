#!/usr/bin/env bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

cd "${REPO_ROOT}"

mkdir -p dist

gcc -o dist/virtual_keyboard virtual_keyboard.c
gcc -o dist/keyboard_writer keyboard_writer.c
