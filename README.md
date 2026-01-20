# VirtusDev - Virtual Barcode Scanner (Rust Edition)

A virtual barcode scanner with modern GUI that emulates a USB barcode reader as a HID keyboard device on Linux.

## Features

- 🎨 **Native GNOME interface** using GTK4
- ⚡ **115200 baud** barcode scanner emulation
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

## Architecture

```
src/
├── main.rs       # GUI application (GTK4)
└── device.rs     # VirtualKeyboard, keymap, event emission
```

**Key Components:**
- `VirtualKeyboard`: Wraps `evdev::uinput::VirtualDevice`
- `Arc<Mutex<>>`: Thread-safe device access for async scans
- `glib::spawn_future`: Non-blocking scan execution with GTK4

## Supported Characters

- **Alphanumeric**: a-z, A-Z, 0-9
- **Special**: space, enter, tab, `-_=+[]{};:'"<>,./?|\`~`
- **Automatic shift**: Uppercase and shifted symbols

## Testing

### Test without GUI (Text Input)

```bash
# Terminal 1: Start a text editor
nano test.txt

# Terminal 2: Run virtusdev and enter barcodes in the GUI
sudo ./target/release/virtusdev
```

### Monitor Device Events

```bash
# Find the event device
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"

# Monitor events (requires sudo)
sudo evtest /dev/input/eventX
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
