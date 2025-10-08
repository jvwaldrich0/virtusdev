#!/bin/bash

echo "╔═══════════════════════════════════════════════════════╗"
echo "║   Virtual Keyboard Output Test                        ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""

if [ "$EUID" -ne 0 ]; then 
    echo "❌ ERROR: Please run as root (sudo)"
    exit 1
fi

DEVICE_PATH=$(/home/jvwaldrich0/Workspace/waldrich.lu/virtus_keyboard/scripts/get_device_path.sh 2>/dev/null)

if [ $? -ne 0 ]; then
    echo "❌ ERROR: Virtual keyboard device not found"
    echo "   Please start virtual_keyboard first:"
    echo "   sudo ./virtual_keyboard"
    exit 1
fi

echo "✅ Device found: $DEVICE_PATH"
echo ""
echo "Testing Options:"
echo ""
echo "Option 1: Test with a text editor"
echo "   1. Open gedit, kate, or any text editor"
echo "   2. Click inside the text area"
echo "   3. Press Enter to continue this script"
echo "   4. Watch the barcode appear in your text editor!"
echo ""
echo "Option 2: Test with a terminal"
echo "   1. Open a new terminal window"
echo "   2. Type: cat"
echo "   3. Press Enter to continue this script"
echo "   4. Watch the barcode appear in the terminal!"
echo ""
echo "Option 3: Test with evtest (see raw events)"
echo "   1. This option will show you the raw keyboard events"
echo "   2. No application focus needed"
echo ""

read -p "Choose option (1/2/3): " choice

case $choice in
    1|2)
        echo ""
        echo "Instructions:"
        echo "   1. Click/focus on your text editor or terminal"
        echo "   2. Make sure cursor is visible and ready for input"
        echo "   3. Press Enter here when ready..."
        read -p ""
        
        echo ""
        echo "Sending test barcode: 1234567890"
        echo ""
        
        /home/jvwaldrich0/Workspace/waldrich.lu/virtus_keyboard/keyboard_writer "$DEVICE_PATH" "1234567890"
        
        echo ""
        echo "✅ Barcode sent!"
        echo ""
        echo "Did you see '1234567890' appear in your application?"
        echo "If YES: ✅ Virtual keyboard is working!"
        echo "If NO: See troubleshooting below"
        echo ""
        echo "Troubleshooting:"
        echo "   - Make sure the application had focus (cursor was blinking)"
        echo "   - Try Option 3 to see raw events"
        echo "   - Check: xev | grep KeyPress  (in another terminal)"
        ;;
    3)
        echo ""
        echo "Raw Event Monitor (Press Ctrl+C to stop)"
        echo ""
        echo "Now sending test barcode in 3 seconds..."
        echo "Watch for KEY events below:"
        echo ""
        
        timeout 10 evtest "$DEVICE_PATH" &
        EVTEST_PID=$!
        
        sleep 3
        
        /home/jvwaldrich0/Workspace/waldrich.lu/virtus_keyboard/keyboard_writer "$DEVICE_PATH" "TEST" 2>&1 | grep -v "BARCODE"
        
        wait $EVTEST_PID
        
        echo ""
        echo "✅ If you saw KEY events above, the keyboard is working!"
        ;;
    *)
        echo "Invalid option"
        exit 1
        ;;
esac

echo ""
echo "Tips:"
echo "   - The virtual keyboard sends input to the FOCUSED application"
echo "   - Click on a text editor/terminal before sending barcodes"
echo "   - Apps can also read events directly using evdev"
echo ""
