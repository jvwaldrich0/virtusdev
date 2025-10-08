# VirtUSDev - Virtual USB Device Emulator

A flexible virtual device emulator that can emulate multiple USB device types on Linux including barcode scanners, keyboards, mice, and serial ports.

## Overview

VirtUSDev allows you to create virtual USB devices that:
- Connect as `/dev/input/eventX` (like real USB devices)
- Support multiple device types (barcode scanner, keyboard, mouse, RS232)
- Work with any application that accepts input from these devices
- Can be configured via command-line, config file, or interactive menu

## Supported Devices

1. **Barcode Scanner** (115200 baud) - Emulates USB HID barcode reader
2. **USB Keyboard** - Full featured virtual USB HID keyboard
3. **Mouse Jiggler** - Prevents screen lock by moving mouse periodically
4. **RS232 Serial Port** - Virtual serial port communication interface

## Created by
- **User**: jvwaldrich0
- **Date**: 2025-10-08 12:59:34 UTC

## Quick Start

### 1. Build
```bash
make
```

### 2. Start a Virtual Device

**Command-line selection:**
```bash
sudo ./virtual_keyboard barcode        # Start barcode scanner
sudo ./virtual_keyboard usb_keyboard   # Start USB keyboard
sudo ./virtual_keyboard mouse_jiggle   # Start mouse jiggler
sudo ./virtual_keyboard rs232          # Start RS232 port
```

**Interactive menu:**
```bash
sudo ./virtual_keyboard
```
You'll see a menu to select your device:
```
╔═══════════════════════════════════════════════════════╗
║           VirtUSDev - Device Selection Menu           ║
╚═══════════════════════════════════════════════════════╝

Select a device to emulate:

  [1] barcode        
      Barcode Scanner (115200 baud) - Emulates USB HID barcode reader

  [2] usb_keyboard   
      USB Keyboard - Full featured virtual USB HID keyboard

  [3] mouse_jiggle   
      Mouse Jiggler - Prevents screen lock by moving mouse periodically

  [4] rs232          
      RS232 Serial Port - Virtual serial port communication interface

Enter choice [1-4]:
```

**Configuration file:**
Create `/etc/virtusdev/config`:
```bash
# VirtUSDev Configuration
DEVICE=barcode
```
Then run:
```bash
sudo ./virtual_keyboard  # Uses device from config
```

### 3. Find Your Device
```bash
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"
```

Look for the `event` number (e.g., `event25`)

### 4. Use the Device (Barcode Scanner Example)

**Interactive mode:**
```bash
sudo ./virtusdev /dev/input/event25
SCAN> 1234567890
SCAN> ABC-123-XYZ
```

**Pipe mode:**
```bash
echo "1234567890" | sudo ./virtusdev /dev/input/event25
```

**Single scan:**
```bash
sudo ./virtusdev /dev/input/event25 "BARCODE123"
```

## Installation

### System-wide Installation
```bash
sudo make install
```

This will:
- Install binaries to `/usr/local/bin/`
- Create config directory at `/etc/virtusdev/`
- Install systemd service file

### Enable Auto-start Service
```bash
# Edit config to set your preferred device
sudo nano /etc/virtusdev/config

# Enable and start service
sudo systemctl enable virtusdev
sudo systemctl start virtusdev

# Check status
sudo systemctl status virtusdev
```

### Uninstall
```bash
sudo make uninstall
```

## Usage Examples (Barcode Scanner)

### Scan into a text file
```bash
# Terminal 1
cat > scanned_items.txt

# Terminal 2
echo "ITEM001" | sudo ./virtusdev /dev/input/event25
echo "ITEM002" | sudo ./virtusdev /dev/input/event25
```

### Scan into a web form
1. Open browser and focus on input field
2. Run: `sudo ./virtusdev /dev/input/event25`
3. Type barcode and press Enter
4. Watch it appear in the browser!

### Automated scanning from file
```bash
cat barcodes.txt | sudo ./virtusdev /dev/input/event25
```

### Monitor barcode scans
```bash
# Terminal 1 - Monitor
sudo evtest /dev/input/event25

# Terminal 2 - Scan
sudo ./virtusdev /dev/input/event25
```

## Configuration

### Config File Format
Location: `/etc/virtusdev/config`

```bash
# VirtUSDev Configuration
# Select device: barcode, usb_keyboard, mouse_jiggle, rs232
DEVICE=barcode
```

### Priority Order
1. Command-line argument (highest priority)
2. Config file setting
3. Interactive menu (if nothing specified)

## Device Details

### Barcode Scanner
- **Baudrate**: 115200 bps
- **Character timing**: ~87 μs per character
- **Supports**: All alphanumeric and common special characters
- **Appends**: Enter key after each scan
- **Compatible with**: UPC/EAN, Code 39, Code 128, QR codes

### USB Keyboard
- Full HID keyboard support
- All standard keys supported
- LED indicators support

### Mouse Jiggler
- Prevents screen lock and sleep
- Minimal mouse movement
- Auto-jiggle mode

### RS232 Serial Port
- Virtual serial communication
- 115200 baud default
- Standard serial port emulation

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

## Command-Line Reference

### virtual_keyboard
```bash
Usage: virtual_keyboard [DEVICE_TYPE]

Available devices:
  barcode         - Barcode Scanner (115200 baud)
  usb_keyboard    - USB Keyboard  
  mouse_jiggle    - Mouse Jiggler
  rs232           - RS232 Serial Port

Examples:
  ./virtual_keyboard barcode        # Start barcode scanner
  ./virtual_keyboard usb_keyboard   # Start USB keyboard
  ./virtual_keyboard                # Interactive menu
```

### virtusdev (formerly keyboard_writer)
```bash
Usage: virtusdev [/dev/input/eventX] [barcode_data]

Modes:
  Interactive: virtusdev /dev/input/eventX
  Pipe:       echo 'data' | virtusdev /dev/input/eventX
  Single:     virtusdev /dev/input/eventX 'data'
```

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
sudo ./virtusdev /dev/input/event25

# Or add yourself to input group
sudo usermod -a -G input $USER
```

### Wrong device number
```bash
# Device number changes after reboot, always check:
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200" | grep event
```

### Service not starting
```bash
# Check service status
sudo systemctl status virtusdev

# View logs
sudo journalctl -u virtusdev -f

# Verify config
cat /etc/virtusdev/config
```

## Technical Specifications

- **Baudrate**: 115200 bps (for barcode/RS232)
- **Interface**: USB HID (emulated via uinput)
- **Character timing**: ~87 μs per character
- **Scan speed**: ~11,500 characters/second theoretical max
- **Protocol**: Keyboard wedge (HID keyboard)
- **Color output**: ANSI terminal colors for better readability