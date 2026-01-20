# Virtual Barcode Reader Device (115200 baud)

A virtual barcode scanner that emulates a USB barcode reader connected as a HID keyboard device on Linux.

## Overview

This emulates a **barcode scanner** that:
- Connects as `/dev/input/eventX` (like real USB barcode scanners)
- Operates at **115200 baud**
- Sends scanned data as keyboard input followed by Enter
- Works with any application that accepts keyboard input

## Created by
- **User**: jvwaldrich0
- **Date**: 2025-10-08 12:59:34 UTC

## Quick Start

### 1. Build
```bash
make
```

### 2. Start the Virtual Device
```bash
sudo ./virtual_keyboard
```

Output:
```
╔═══════════════════════════════════════════════════════╗
║      VIRTUAL BARCODE READER DEVICE CREATED           ║
╚═══════════════════════════════════════════════════════╝
  Device Name  : Virtual Keyboard 115200
  Device Type  : Barcode Scanner (HID Keyboard)
  Baudrate     : 115200 bps
```

### 3. Find Your Device
```bash
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"
```

Look for the `event` number (e.g., `event25`)

### 4. Scan Barcodes

**Interactive mode:**
```bash
sudo ./keyboard_writer /dev/input/event25
SCAN> 1234567890
SCAN> ABC-123-XYZ
```

**Pipe mode:**
```bash
echo "1234567890" | sudo ./keyboard_writer /dev/input/event25
```

**Single scan:**
```bash
sudo ./keyboard_writer /dev/input/event25 "BARCODE123"
```

## Usage Examples

### Scan into a text file
```bash
# Terminal 1
cat > scanned_items.txt

# Terminal 2
echo "ITEM001" | sudo ./keyboard_writer /dev/input/event25
echo "ITEM002" | sudo ./keyboard_writer /dev/input/event25
```

### Scan into a web form
1. Open browser and focus on input field
2. Run: `sudo ./keyboard_writer /dev/input/event25`
3. Type barcode and press Enter
4. Watch it appear in the browser!

### Automated scanning from file
```bash
cat barcodes.txt | sudo ./keyboard_writer /dev/input/event25
```

### Monitor barcode scans
```bash
# Terminal 1 - Monitor
sudo evtest /dev/input/event25

# Terminal 2 - Scan
sudo ./keyboard_writer /dev/input/event25
```

## Real Barcode Scanner Behavior

Like a real barcode scanner, this device:
1. ✅ Sends data as keyboard input
2. ✅ Automatically appends Enter/Return after each scan
3. ✅ Works at 115200 baud rate
4. ✅ Supports alphanumeric and special characters
5. ✅ Can be used with any application (POS, inventory, web forms)

## Supported Barcode Formats

The device sends plain text, supporting common barcode formats:
- **UPC/EAN**: `012345678905`
- **Code 39**: `*ABC123*`
- **Code 128**: `ABC-123-xyz`
- **QR Code Data**: `http://example.com`
- **Custom formats**: Any alphanumeric string

## Device Information

```bash
# Check device details
cat /proc/bus/input/devices | grep -A 10 "Virtual Keyboard 115200"

# Monitor events
sudo evtest /dev/input/event25

# Check permissions
ls -l /dev/input/event25
```

## Troubleshooting

### Device not found
```bash
# Make sure virtual_keyboard is running
ps aux | grep virtual_keyboard

# Load uinput module
sudo modprobe uinput
```

### Permission denied
```bash
# Run with sudo
sudo ./keyboard_writer /dev/input/event25

# Or add yourself to input group
sudo usermod -a -G input $USER
```

### Wrong device number
```bash
# Device number changes after reboot, always check:
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200" | grep event
```

## Technical Specifications

- **Baudrate**: 115200 bps
- **Interface**: USB HID (emulated via uinput)
- **Character timing**: ~87 μs per character
- **Scan speed**: ~11,500 characters/second theoretical max
- **Protocol**: Keyboard wedge (HID keyboard)