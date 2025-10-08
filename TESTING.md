# Testing the Virtual Keyboard at 115200 Baud

## Quick Test

1. **Terminal 1** - Create the virtual keyboard:
```bash
sudo ./virtual_keyboard
```

You should see:
```
═══════════════════════════════════════════════════════
  Virtual Keyboard Device Created
═══════════════════════════════════════════════════════
  Device Name    : Virtual Keyboard 115200
  Baudrate       : 115200 bps
  Bit Time       : 8.68 μs
  Char Time      : 86.81 μs
  Vendor ID      : 0x1234
  Product ID     : 0x5678
═══════════════════════════════════════════════════════
```

2. **Terminal 2** - Monitor keyboard events:
```bash
# Find the event device number
cat /proc/bus/input/devices | grep -A 5 "Virtual Keyboard 115200"

# Monitor events (replace X with your event number)
sudo evtest /dev/input/eventX
```

3. **Terminal 3** - Write to the keyboard:
```bash
sudo ./keyboard_writer
# Type: Hello World
# Press Enter
```

You should see timing information showing the baudrate-based delays!

## Baudrate Verification

The timing should respect the 115200 baud rate:
- Each character takes approximately 86.8 μs to transmit
- Including inter-character delays, expect ~260 μs per character total
- For "Hello World\n" (12 chars): ~3.1 ms total

## Performance Test

```bash
# Terminal 3
sudo ./keyboard_writer
> ABCDEFGHIJKLMNOPQRSTUVWXYZ
# Watch the timing output to verify baudrate compliance
```

## Testing with Real Applications

### Test with `evtest`
```bash
# Monitor the actual events with timestamps
sudo evtest /dev/input/eventX
```

### Test Device Properties
```bash
# Check device information
cat /proc/bus/input/devices | grep -A 10 "Virtual Keyboard 115200"

# List all input devices
ls -la /dev/input/by-id/
```

## Advanced: Baudrate Timing Analysis

Create a test file to analyze timing:

```bash
# Terminal 3
sudo ./keyboard_writer
> The quick brown fox jumps over the lazy dog
```

The output will show:
- Number of characters sent
- Baudrate used (115200)
- Total transmission time
- This should align with baudrate calculations