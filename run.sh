#!/bin/bash
set -e

# Get absolute path of the binary
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BINARY="$SCRIPT_DIR/target/release/virtusdev"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found. Building first..."
    ./build.sh
fi

echo "🚀 Starting VirtusDev Virtual Barcode Scanner..."
echo ""

# Check if pkexec is available (better for GUI apps)
if command -v pkexec > /dev/null 2>&1; then
    echo "Using pkexec for privilege escalation..."
    pkexec env DISPLAY="$DISPLAY" \
              XAUTHORITY="$XAUTHORITY" \
              XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" \
              HOME="$HOME" \
              LIBGL_ALWAYS_SOFTWARE=1 \
              WINIT_UNIX_BACKEND=x11 \
              WINIT_X11_SCALE_FACTOR=1 \
              __GLX_VENDOR_LIBRARY_NAME=mesa \
              "$BINARY"
else
    # Fallback to sudo with X11 configuration
    xhost +local:root > /dev/null 2>&1
    
    # Create temporary XAUTHORITY for root if needed
    if [ -n "$XAUTHORITY" ] && [ -f "$XAUTHORITY" ]; then
        TEMP_XAUTH="/tmp/.virtusdev-xauth-$$"
        cp "$XAUTHORITY" "$TEMP_XAUTH"
        chmod 644 "$TEMP_XAUTH"
        XAUTH_TO_USE="$TEMP_XAUTH"
    else
        XAUTH_TO_USE="$HOME/.Xauthority"
    fi
    
    sudo -E DISPLAY="$DISPLAY" \
           XAUTHORITY="$XAUTH_TO_USE" \
           XDG_RUNTIME_DIR="$XDG_RUNTIME_DIR" \
           HOME="$HOME" \
           LIBGL_ALWAYS_SOFTWARE=1 \
           WINIT_UNIX_BACKEND=x11 \
           WINIT_X11_SCALE_FACTOR=1 \
           __GLX_VENDOR_LIBRARY_NAME=mesa \
           "$BINARY"
    
    # Cleanup
    [ -n "${TEMP_XAUTH:-}" ] && rm -f "$TEMP_XAUTH"
    xhost -local:root > /dev/null 2>&1
fi
