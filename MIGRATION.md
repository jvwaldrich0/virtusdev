# Migration Guide

## Upgrading from Previous Versions

### Changes in VirtUSDev 2.0

VirtUSDev has been significantly enhanced to support multiple device types. Here's what changed:

#### Renamed Files
- `keyboard_writer` → `virtusdev` (binary name)
- `keyboard_writer.c` → `virtusdev.c` (source file, legacy file kept for reference)

**Backward Compatibility:** A symlink `keyboard_writer` → `virtusdev` is created for backward compatibility.

#### New Features
1. **Multi-device support**: Choose from barcode, USB keyboard, mouse jiggler, or RS232
2. **Interactive menu**: Run without parameters to select device
3. **Configuration file**: Set default device in `/etc/virtusdev/config`
4. **Colored output**: Better readability with ANSI colors
5. **System installation**: Install to `/usr/local/bin` with `make install`
6. **Systemd service**: Auto-start on boot

#### Updated Commands

**Old way:**
```bash
sudo ./virtual_keyboard              # Only barcode scanner
sudo ./keyboard_writer /dev/input/event25
```

**New way (recommended):**
```bash
sudo ./virtual_keyboard barcode      # Select device type
sudo ./virtusdev /dev/input/event25
```

**New way (backward compatible):**
```bash
sudo ./virtual_keyboard              # Still works (shows menu)
sudo ./keyboard_writer /dev/input/event25  # Symlink still works
```

#### Device Architecture

Previous version:
- Single barcode scanner device hardcoded in `virtual_keyboard.c`

Current version:
- Modular device system in `devices/` directory
- Each device type in separate file
- Easy to add new devices

```
devices/
├── barcode.c         # Barcode scanner (original functionality)
├── usb_keyboard.c    # USB keyboard
├── mouse_jiggle.c    # Mouse jiggler
├── rs232.c           # RS232 serial port
└── device_interface.h
```

#### Configuration File

New in version 2.0, you can set a default device:

Create `/etc/virtusdev/config`:
```bash
# VirtUSDev Configuration
DEVICE=barcode
```

Priority:
1. Command-line argument (highest)
2. Config file
3. Interactive menu (lowest)

#### Installation

**Old way:**
```bash
make
sudo cp virtual_keyboard /usr/local/bin/
sudo cp keyboard_writer /usr/local/bin/
```

**New way:**
```bash
make
sudo make install
```

This installs:
- `/usr/local/bin/virtual_keyboard`
- `/usr/local/bin/virtusdev`
- `/usr/local/bin/keyboard_writer` (symlink)
- `/etc/virtusdev/config` (if doesn't exist)
- `/etc/systemd/system/virtusdev.service`

#### Systemd Service

New feature - auto-start on boot:

```bash
# Edit config
sudo nano /etc/virtusdev/config

# Enable service
sudo systemctl enable virtusdev
sudo systemctl start virtusdev

# Check status
sudo systemctl status virtusdev
```

### Upgrading Scripts

If you have scripts using the old commands, they should continue to work due to backward compatibility.

**Example - No changes needed:**
```bash
#!/bin/bash
# This script still works unchanged
sudo ./keyboard_writer /dev/input/event25 "BARCODE123"
```

**Example - Updated to use new features:**
```bash
#!/bin/bash
# Modern version with colors and error handling
if ! virtusdev /dev/input/event25 "BARCODE123"; then
    echo "Error: Failed to send barcode"
    exit 1
fi
```

### Breaking Changes

None! All existing functionality is preserved and backward compatible.

### Deprecated Features

- `keyboard_writer.c` source file (replaced by `virtusdev.c`, but kept for reference)
- Building with old script method (use Makefile instead)

### Recommendations

1. **Use new command names** in new scripts: `virtusdev` instead of `keyboard_writer`
2. **Use `make install`** for system-wide installation
3. **Set up systemd service** if you need auto-start
4. **Use config file** to avoid typing device name repeatedly
5. **Update documentation** to reference new tool names

### Getting Help

```bash
# Device emulator help
./virtual_keyboard --help

# Barcode writer help
./virtusdev --help

# Check version
git log --oneline | head -1
```

### Rollback

If you need to rollback to the previous version:

```bash
git checkout <previous-commit>
make clean
make
```

The original `keyboard_writer.c` is still in the repository for reference.
