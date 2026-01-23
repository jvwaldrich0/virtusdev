# VirtusDev - Virtual Input Device Emulator (Rust Edition)

A comprehensive virtual input device emulator with modern GUI that emulates USB devices including barcode scanners, bill validators, and coin acceptors as HID keyboard devices on Linux.

## Features

- 🎨 **Native GNOME interface** using GTK4
- ⚡ **115200 baud** barcode scanner emulation
- 💰 **Bill and coin emulator** with serial bridge support
- 📊 **Real-time scan history** with duration tracking
- 🖥️ **Native GUI** with system theme support
- 🔒 **Safe Rust** implementation with proper error handling

## Quick Start

**All dependencies (Rust, system libraries) are already installed!**

### Just run:

```bash
./run.sh
```

This will:
1. Build the project (if not already built)
2. Launch the GUI with sudo (required for `/dev/uinput` access)

### Manual build:

```bash
./build.sh
sudo ./target/release/virtusdev
```

### 3. Use the GUI

1. Enter a barcode in the input field
2. Click **SCAN** or press Enter
3. The barcode will be sent as keyboard input to the focused application
4. View scan history with timing information

## Device Information

The GUI displays:
- **Device Name**: Virtual Keyboard 115200
- **Event Path**: `/dev/input/eventX` (auto-detected)
- **Baudrate**: 115200 bps
- **Vendor ID**: 0x1234
- **Product ID**: 0x5678
- **Status**: Running / Scanning indicator
- **Recent Scans**: Last 10 scans with duration

## Permissions

### Option 1: Run with sudo (Quick)

```bash
sudo ./target/release/virtusdev
```

### Option 2: Add user to input group (Permanent)

```bash
# Add your user to the input group
sudo usermod -a -G input $USER

# Create udev rule
echo 'KERNEL=="uinput", MODE="0660", GROUP="input"' | sudo tee /etc/udev/rules.d/99-uinput.rules

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger

# Log out and back in for group changes to take effect
```

## How It Works

1. Creates a virtual input device via Linux `uinput` kernel module
2. Registers as a USB HID keyboard (`/dev/input/eventX`)
3. Sends keyboard events for each character in the barcode
4. Automatically appends Enter key after each scan
5. Timing matches 115200 baud rate behavior

## Bill & Coin Emulator

The emulator provides comprehensive support for vending machine and payment system testing through the eSSP (Encrypted Serial Protocol):

### Features

**Note Validator (NV200)**
- Simulates an ICT NV200 bill validator
- Supports BRL currency (Brazilian Real)
- 7 denominations: R$2, R$5, R$10, R$20, R$50, R$100, R$200
- Protocol 6 eSSP compliance
- Configurable balances for testing

**Coin Acceptor (Smart Hopper)**
- Simulates coin acceptance and payout
- USD currency support
- 6 denominations: 1¢, 5¢, 10¢, 25¢, 50¢, $1.00
- Full payout simulation with event tracking

**Serial Bridge (PTY)**
- Virtual serial port (PTY) for device communication
- Auto-creates `/dev/pts/X` for payment system integration
- Transparent eSSP protocol handling
- Real-time transaction logging

### Usage

```bash
# Run the bill emulator with serial bridge
./target/release/bill_emulator

# The emulator will display:
# ✓ Virtual serial port created: /dev/pts/X
# Configure payment system to use: /dev/pts/X
```

### Supported eSSP Commands

- `SYNC` - Device synchronization
- `SETUP_REQUEST` - Device information query
- `POLL` - Event polling and status
- `ENABLE` / `DISABLE` - Device control
- `SET_INHIBITS` - Denomination masking
- `GET_ALL_LEVELS` - Current inventory
- `PAYOUT` - Coin dispensing simulation
- `REJECT` - Note rejection
- `SET_ROUTE` - Stacking/routing control

### Event Codes

- `RESET` (0xF1) - Device reset
- `READ` (0xEF) - Note detected
- `CREDIT` (0xEE) - Note accepted
- `STACKING` (0xCC) - Note being stacked
- `STACKED` (0xEB) - Note stacked
- `DISPENSING` (0xDA) - Coin dispensing
- `DISPENSED` (0xD2) - Coin dispensed
- `COINS_VALUE_ADDED` (0xBF) - Coin credit received

## Architecture

```
src/
├── main.rs             # GUI application (GTK4)
├── device.rs           # VirtualKeyboard, keymap, event emission
├── bill_emulator.rs    # Device state & eSSP command handling
├── essp_protocol.rs    # eSSP packet encoding/decoding with CRC-16
├── serial_bridge.rs    # PTY serial port & device communication
└── bin/
    ├── bill_emulator   # Standalone bill/coin emulator
    └── crc_check       # CRC validation utility
```

**Key Components:**
- `VirtualKeyboard`: Wraps `evdev::uinput::VirtualDevice`
- `Arc<Mutex<>>`: Thread-safe device access for async scans
- `glib::spawn_future`: Non-blocking scan execution with GTK4
- `DeviceState`: Bill validator and coin device emulation
- `EsspPacket`: Protocol-compliant packet serialization with CRC-16
- `SerialBridge`: PTY-based serial communication

## Supported Characters

- **Alphanumeric**: a-z, A-Z, 0-9
- **Special**: space, enter, tab, `-_=+[]{};:'"<>,./?|\`~`
- **Automatic shift**: Uppercase and shifted symbols

## Testing

### Barcode Scanner (GUI)

```bash
# Terminal 1: Start a text editor
nano test.txt

# Terminal 2: Run virtusdev and enter barcodes in the GUI
sudo ./target/release/virtusdev
```

### Monitor Keyboard Events

```bash
# Find the event device
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"

# Monitor events (requires sudo)
sudo evtest /dev/input/eventX
```

### Bill & Coin Emulator Testing

```bash
# Terminal 1: Start the emulator
./target/release/bill_emulator

# Terminal 2: Connect using a payment system or test client
# Using socat for testing:
socat - /dev/pts/X

# Send raw eSSP commands and monitor responses
# Check /tmp/emu_cmd.log for transaction logs
tail -f /tmp/emu_cmd.log
```

### Protocol Validation

```bash
# Verify CRC calculations
./target/release/crc_check
```

## Legacy C Version

The original C implementation is preserved in `legacy/` directory for reference.

## Troubleshooting

### "Permission denied: /dev/uinput"
Run with `sudo` or follow the permanent permissions setup above.

### Device not appearing
```bash
# Check if uinput module is loaded
lsmod | grep uinput

# Load if missing
sudo modprobe uinput
```

### Build errors
Ensure all system dependencies are installed:
```bash
sudo apt-get install -y pkg-config build-essential libgtk-4-dev
```

## Created By

**jvwaldrich0**

---

## Technical Specifications

- **Baudrate**: 115200 bps
- **Interface**: USB HID (emulated via uinput)
- **Character timing**: ~87 μs per character
- **Protocol**: Keyboard wedge (HID keyboard)
- **GUI Framework**: GTK4 (gtk4-rs 0.9)
- **Language**: Rust (edition 2021)
