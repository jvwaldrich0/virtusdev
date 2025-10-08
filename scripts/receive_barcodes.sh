#!/bin/bash

DEVICE="/dev/input/event25"

echo "╔═══════════════════════════════════════════════════════╗"
echo "║         BARCODE RECEIVER - evdev Reader               ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
echo "📋 Device: $DEVICE"
echo ""
echo "🎯 Instructions:"
echo "   1. Keep this terminal open"
echo "   2. In ANOTHER terminal, send barcodes:"
echo "      sudo dist/keyboard_writer /dev/input/event25 \"YOUR_BARCODE\""
echo ""
echo "   3. Barcodes will appear below!"
echo ""
echo "Press Ctrl+C to stop receiving"
echo ""
echo "═══════════════════════════════════════════════════════"
echo ""

if python3 -c "import evdev" 2>/dev/null; then
    echo "[INFO] Using evdev reader"
    echo ""
    sudo python3 /home/jvwaldrich0/Workspace/waldrich.lu/virtus_keyboard/scripts/test_evdev_reader.py "$DEVICE"
else
    if command -v evtest &> /dev/null; then
        echo "[INFO] Using evtest (raw event display)"
        echo ""
        sudo evtest "$DEVICE"
    else
        echo "❌ ERROR: Neither Python evdev nor evtest is available"
        echo ""
        echo "Install Python evdev:"
        echo "  pip3 install evdev"
        echo ""
        echo "OR install evtest:"
        echo "  sudo apt-get install evtest"
        exit 1
    fi
fi
