# VirtusDev - Quick Start

## Everything is Ready! ✅

All dependencies are installed:
- ✅ Rust 1.91.1
- ✅ System libraries (fontconfig, etc.)
- ✅ Project compiled (release mode)

## Run the Application

```bash
./run.sh
```

That's it! The GUI will launch with sudo privileges.

## What You'll See

A dark-themed window with:
- Device status (Running/Scanning)
- Event path (`/dev/input/eventX`)
- Input field for barcode data
- SCAN button
- History of recent scans with timing
- "Made by jvwaldrich0" footer

## How to Use

1. Enter barcode text in the input field
2. Press Enter or click SCAN
3. The barcode will be sent as keyboard input to any focused application
4. See scan timing in the history

## Executable Location

The compiled binary is at:
```
./target/release/virtusdev
```

Size: ~17MB (includes GUI framework)

## Troubleshooting

**Permission denied?**
The script already uses `sudo`. If it still fails, check:
```bash
ls -l /dev/uinput
```

**Device not working?**
```bash
# Check if device was created
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"
```

---

Created by jvwaldrich0
