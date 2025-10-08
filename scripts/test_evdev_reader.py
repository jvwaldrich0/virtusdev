#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Test Virtual Keyboard Integration with evdev
Reads barcodes from the virtual keyboard device
"""

import sys
import time
from select import select

try:
    from evdev import InputDevice, ecodes, categorize
except ImportError:
    print("ERROR: evdev library not found")
    print("Install with: pip install evdev")
    sys.exit(1)

QWERTY_US_LAYOUT = {
    2: [u'1', u'', u'!', u'!', u'', u'', u''],
    3: [u'2', u'', u'@', u'@', u'', u'', u''],
    4: [u'3', u'', u'#', u'#', u'', u'', u''],
    5: [u'4', u'', u'$', u'$', u'', u'', u''],
    6: [u'5', u'', u'%', u'%', u'', u'', u''],
    7: [u'6', u'', u'^', u'^', u'', u'', u''],
    8: [u'7', u'', u'&', u'&', u'', u'', u''],
    9: [u'8', u'', u'*', u'*', u'', u'', u''],
    10: [u'9', u'', u'(', u'(', u'', u'', u''],
    11: [u'0', u'', u')', u')', u'', u'', u''],
    12: [u'-', u'', u'_', u'_', u'', u'', u''],
    13: [u'=', u'', u'+', u'+', u'', u'', u''],
    16: [u'q', u'', u'Q', u'Q', u'', u'', u''],
    17: [u'w', u'', u'W', u'W', u'', u'', u''],
    18: [u'e', u'', u'E', u'E', u'', u'', u''],
    19: [u'r', u'', u'R', u'R', u'', u'', u''],
    20: [u't', u'', u'T', u'T', u'', u'', u''],
    21: [u'y', u'', u'Y', u'Y', u'', u'', u''],
    22: [u'u', u'', u'U', u'U', u'', u'', u''],
    23: [u'i', u'', u'I', u'I', u'', u'', u''],
    24: [u'o', u'', u'O', u'O', u'', u'', u''],
    25: [u'p', u'', u'P', u'P', u'', u'', u''],
    26: [u'[', u'', u'{', u'{', u'', u'', u''],
    27: [u']', u'', u'}', u'}', u'', u'', u''],
    28: [u'\n', u'', u'', u'', u'', u'', u''],
    30: [u'a', u'', u'A', u'A', u'', u'', u''],
    31: [u's', u'', u'S', u'S', u'', u'', u''],
    32: [u'd', u'', u'D', u'D', u'', u'', u''],
    33: [u'f', u'', u'F', u'F', u'', u'', u''],
    34: [u'g', u'', u'G', u'G', u'', u'', u''],
    35: [u'h', u'', u'H', u'H', u'', u'', u''],
    36: [u'j', u'', u'J', u'J', u'', u'', u''],
    37: [u'k', u'', u'K', u'K', u'', u'', u''],
    38: [u'l', u'', u'L', u'L', u'', u'', u''],
    39: [u';', u'', u':', u':', u'', u'', u''],
    40: [u'\'', u'', u'"', u'"', u'', u'', u''],
    43: [u'\\', u'', u'|', u'|', u'', u'', u''],
    44: [u'z', u'', u'Z', u'Z', u'', u'', u''],
    45: [u'x', u'', u'X', u'X', u'', u'', u''],
    46: [u'c', u'', u'C', u'C', u'', u'', u''],
    47: [u'v', u'', u'V', u'V', u'', u'', u''],
    48: [u'b', u'', u'B', u'B', u'', u'', u''],
    49: [u'n', u'', u'N', u'N', u'', u'', u''],
    50: [u'm', u'\n', u'M', u'M', u'', u'', u''],
    51: [u',', u'', u'<', u'<', u'', u'', u''],
    52: [u'.', u'', u'>', u'>', u'', u'', u''],
    53: [u'/', u'', u'?', u'?', u'', u'', u''],
    57: [u' ', u'', u' ', u' ', u'', u'', u''],
    96: [u'\n', u'', u'', u'', u'', u'', u'']
}

MODIFIERS = {
    0: 0,
    29: 1,
    42: 2,
    54: 3,
    56: 4,
    97: 5,
    100: 6
}


def read_barcode(device_file, timeout=0):
    """
    Read a barcode from the device using evdev
    
    Args:
        device_file: Path to input device (e.g., /dev/input/event25)
        timeout: Timeout in seconds (0 = wait forever)
    
    Returns:
        String containing the barcode
    """
    try:
        device = InputDevice(device_file)
        print(f"✅ Opened device: {device.name}")
        device.grab()  # Exclusive access
    except Exception as e:
        print(f"❌ ERROR: Cannot open device: {e}")
        return None
    
    if timeout > 0:
        select([device], [], [], timeout)
    else:
        select([device], [], [])
    
    ret = ''
    modifier = 0
    
    try:
        while True:
            for event in device.read():
                if event.type == ecodes.EV_KEY:
                    data = categorize(event)
                    if data.keystate == 1:  # KEY_DOWN
                        if data.scancode in MODIFIERS:
                            modifier = data.scancode
                        elif data.scancode in QWERTY_US_LAYOUT:
                            keycode = data.scancode
                            value = QWERTY_US_LAYOUT[keycode][MODIFIERS[modifier]]
                            if value not in ["\r", "\n"]:
                                ret += value
                            elif ret != '':
                                device.ungrab()
                                return ret
                    elif data.keystate == 0:  # KEY_UP
                        if data.scancode in MODIFIERS:
                            modifier = 0
            select([device], [], [], 0.1)
    except IOError as e:
        if e.errno != 11:
            print(f"❌ ERROR: {e}")
            device.ungrab()
            return None
    except KeyboardInterrupt:
        device.ungrab()
        return None
    
    device.ungrab()
    return ret if ret != '' or timeout > 0 else None


def main():
    print("╔═══════════════════════════════════════════════════════╗")
    print("║   Virtual Keyboard evdev Reader Test                 ║")
    print("╚═══════════════════════════════════════════════════════╝")
    print()
    
    if len(sys.argv) < 2:
        print("Usage: sudo python3 test_evdev_reader.py /dev/input/eventX")
        print()
        print("Find device with:")
        print("  sudo ./scripts/get_device_path.sh")
        print()
        sys.exit(1)
    
    device_file = sys.argv[1]
    
    print(f"📋 Device: {device_file}")
    print()
    print("🎯 Test Instructions:")
    print()
    print("1. This script will wait for barcode scans")
    print("2. In another terminal, send a test barcode:")
    print(f"   sudo ./keyboard_writer {device_file} \"1234567890\"")
    print()
    print("3. The barcode should appear below")
    print("4. Press Ctrl+C to exit")
    print()
    print("═══════════════════════════════════════════════════════")
    print()
    
    scan_count = 0
    
    try:
        while True:
            print(f"[SCAN #{scan_count + 1}] Waiting for barcode scan...")
            
            start_time = time.time()
            barcode = read_barcode(device_file, timeout=0)
            end_time = time.time()
            
            if barcode:
                scan_count += 1
                duration = (end_time - start_time) * 1000  # ms
                
                print(f"✅ RECEIVED: '{barcode}'")
                print(f"   Length: {len(barcode)} characters")
                print(f"   Time: {duration:.2f} ms")
                print()
            else:
                print("❌ Failed to read barcode")
                break
                
    except KeyboardInterrupt:
        print()
        print("═══════════════════════════════════════════════════════")
        print(f"📊 Total scans received: {scan_count}")
        print()
        print("✅ Test completed successfully!")


if __name__ == "__main__":
    main()

