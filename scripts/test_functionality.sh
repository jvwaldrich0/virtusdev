#!/bin/bash
# Test script for VirtUSDev functionality

echo "╔═══════════════════════════════════════════════════════╗"
echo "║            VirtUSDev - Functionality Tests            ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[31m'
GREEN='\033[32m'
YELLOW='\033[33m'
RESET='\033[0m'

TESTS_PASSED=0
TESTS_FAILED=0

test_pass() {
    echo -e "${GREEN}✓${RESET} $1"
    ((TESTS_PASSED++))
}

test_fail() {
    echo -e "${RED}✗${RESET} $1"
    ((TESTS_FAILED++))
}

echo "Building project..."
make clean > /dev/null 2>&1
make all > /dev/null 2>&1
if [ $? -eq 0 ]; then
    test_pass "Build successful"
else
    test_fail "Build failed"
    exit 1
fi

echo ""
echo "Testing device selection..."

# Test 1: Help output
./virtual_keyboard --help > /tmp/test_help.txt 2>&1
if grep -q "Available devices:" /tmp/test_help.txt; then
    test_pass "Help command works"
else
    test_fail "Help command failed"
fi

# Test 2: Invalid device
./virtual_keyboard invalid_device > /tmp/test_invalid.txt 2>&1
if grep -q "Unknown device type" /tmp/test_invalid.txt; then
    test_pass "Invalid device detection works"
else
    test_fail "Invalid device detection failed"
fi

# Test 3: Check all devices are listed
for device in barcode usb_keyboard mouse_jiggle rs232; do
    if grep -q "$device" /tmp/test_help.txt; then
        test_pass "Device '$device' is available"
    else
        test_fail "Device '$device' not found"
    fi
done

echo ""
echo "Testing virtusdev (barcode writer)..."

# Test 4: virtusdev help
./virtusdev --help > /tmp/test_virtusdev.txt 2>&1
if grep -q "BARCODE READER EMULATOR" /tmp/test_virtusdev.txt; then
    test_pass "virtusdev help works"
else
    test_fail "virtusdev help failed"
fi

# Test 5: keyboard_writer symlink
make keyboard_writer > /dev/null 2>&1
if [ -L keyboard_writer ]; then
    test_pass "keyboard_writer symlink created"
    
    ./keyboard_writer --help > /tmp/test_kbwriter.txt 2>&1
    if grep -q "BARCODE READER EMULATOR" /tmp/test_kbwriter.txt; then
        test_pass "keyboard_writer symlink works"
    else
        test_fail "keyboard_writer symlink doesn't work"
    fi
else
    test_fail "keyboard_writer symlink not created"
fi

echo ""
echo "Testing file structure..."

# Test 6: Device files exist
for device_file in devices/barcode.c devices/usb_keyboard.c devices/mouse_jiggle.c devices/rs232.c; do
    if [ -f "$device_file" ]; then
        test_pass "Device file $device_file exists"
    else
        test_fail "Device file $device_file missing"
    fi
done

# Test 7: Object files compiled
for obj_file in devices/barcode.o devices/usb_keyboard.o devices/mouse_jiggle.o devices/rs232.o; do
    if [ -f "$obj_file" ]; then
        test_pass "Object file $obj_file compiled"
    else
        test_fail "Object file $obj_file not compiled"
    fi
done

echo ""
echo "Testing make targets..."

# Test 8: make check
make check > /tmp/test_check.txt 2>&1
if grep -q "All checks passed" /tmp/test_check.txt; then
    test_pass "make check passes"
else
    test_fail "make check failed"
fi

echo ""
echo "╔═══════════════════════════════════════════════════════╗"
echo "║                    Test Summary                       ║"
echo "╚═══════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}Passed:${RESET} $TESTS_PASSED"
echo -e "${RED}Failed:${RESET} $TESTS_FAILED"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${RESET}"
    exit 0
else
    echo -e "${RED}Some tests failed!${RESET}"
    exit 1
fi
