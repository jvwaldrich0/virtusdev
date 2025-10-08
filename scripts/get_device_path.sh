#!/bin/bash

DEVICE_NAME="Virtual Keyboard 115200"

EVENT_NUM=$(cat /proc/bus/input/devices | grep -A 5 "$DEVICE_NAME" | grep -o 'event[0-9]*' | tail -1)

if [ -z "$EVENT_NUM" ]; then
    echo "ERROR: Virtual keyboard device not found" >&2
    echo "Make sure virtual_keyboard is running" >&2
    exit 1
fi

echo "/dev/input/$EVENT_NUM"
