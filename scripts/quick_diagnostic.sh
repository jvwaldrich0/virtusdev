#!/bin/bash

DEVICE="/dev/input/event25"

echo "═══════════════════════════════════════════════════════"
echo "Virtual Keyboard Quick Diagnostic"
echo "═══════════════════════════════════════════════════════"
echo ""

if [ ! -e "$DEVICE" ]; then
    echo "❌ ERROR: Device $DEVICE does not exist"
    echo "   Run: sudo ./virtual_keyboard"
    exit 1
fi

echo "✅ Device exists: $DEVICE"
echo ""

if ! command -v evtest &> /dev/null; then
    echo "⚠️  evtest not installed (optional for diagnostics)"
    echo "   Install with: sudo apt-get install evtest"
    echo ""
fi

echo "📊 Processes using the device:"
sudo lsof "$DEVICE" 2>/dev/null | grep -v "COMMAND" | awk '{print "   - " $1 " (PID: " $2 ")"}'
echo ""

if sudo lsof "$DEVICE" 2>/dev/null | grep -q "Xorg"; then
    echo "✅ Xorg is reading the device (input will go to focused app)"
else
    echo "⚠️  Xorg is NOT reading the device"
    echo "   This might mean input won't appear in GUI applications"
fi

echo ""
echo "🎯 To see output on screen:"
echo ""
echo "Method 1 - Text Editor Test:"
echo "   1. Open: gedit"
echo "   2. Click in the editor (cursor blinking)"
echo "   3. Run: sudo ./keyboard_writer $DEVICE \"TEST123\""
echo "   4. Watch it appear in gedit!"
echo ""
echo "Method 2 - Terminal Test:"
echo "   # Terminal 1"
echo "   cat"
echo ""  
echo "   # Terminal 2"
echo "   sudo ./keyboard_writer $DEVICE \"TEST123\""
echo ""
echo "Method 3 - Direct Read (evdev):"
echo "   sudo python3 ./scripts/test_evdev_reader.py $DEVICE"
echo "   # Then in another terminal:"
echo "   sudo ./keyboard_writer $DEVICE \"TEST123\""
echo ""
echo "═══════════════════════════════════════════════════════"
