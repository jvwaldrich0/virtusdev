CC = gcc
CFLAGS = -Wall -Wextra -O2
TARGETS = virtual_keyboard virtusdev
DEVICE_OBJS = devices/barcode.o devices/usb_keyboard.o devices/mouse_jiggle.o devices/rs232.o
PREFIX ?= /usr/local
SYSCONFDIR ?= /etc

all: $(TARGETS)

devices/%.o: devices/%.c devices/%.h devices/device_interface.h device_config.h
	$(CC) $(CFLAGS) -c -o $@ $<

virtual_keyboard: virtual_keyboard.c device_config.h $(DEVICE_OBJS)
	$(CC) $(CFLAGS) -o virtual_keyboard virtual_keyboard.c $(DEVICE_OBJS)

virtusdev: virtusdev.c device_config.h
	$(CC) $(CFLAGS) -o virtusdev virtusdev.c

# Legacy keyboard_writer target for compatibility
keyboard_writer: virtusdev
	ln -sf virtusdev keyboard_writer

check: $(TARGETS)
	@echo "Running basic checks..."
	@echo "[1/4] Checking virtual_keyboard binary..."
	@test -f virtual_keyboard || (echo "ERROR: virtual_keyboard not built" && exit 1)
	@echo "[2/4] Checking virtusdev binary..."
	@test -f virtusdev || (echo "ERROR: virtusdev not built" && exit 1)
	@echo "[3/4] Checking file permissions..."
	@test -x virtual_keyboard || (echo "ERROR: virtual_keyboard not executable" && exit 1)
	@test -x virtusdev || (echo "ERROR: virtusdev not executable" && exit 1)
	@echo "[4/4] Checking device modules..."
	@test -f devices/barcode.o || (echo "ERROR: device modules not built" && exit 1)
	@echo "✅ All checks passed!"

clean:
	rm -f $(TARGETS) keyboard_writer
	rm -f devices/*.o

install: all
	@echo "Installing binaries..."
	install -d $(PREFIX)/bin
	install -m 755 virtual_keyboard $(PREFIX)/bin/
	install -m 755 virtusdev $(PREFIX)/bin/
	ln -sf virtusdev $(PREFIX)/bin/keyboard_writer
	@echo "Creating config directory..."
	install -d $(SYSCONFDIR)/virtusdev
	@if [ ! -f $(SYSCONFDIR)/virtusdev/config ]; then \
		echo "# VirtUSDev Configuration" > $(SYSCONFDIR)/virtusdev/config; \
		echo "# Select device: barcode, usb_keyboard, mouse_jiggle, rs232" >> $(SYSCONFDIR)/virtusdev/config; \
		echo "DEVICE=barcode" >> $(SYSCONFDIR)/virtusdev/config; \
		echo "Created default config at $(SYSCONFDIR)/virtusdev/config"; \
	fi
	@echo "Installing systemd service..."
	@if [ -d /etc/systemd/system ]; then \
		install -m 644 virtusdev.service /etc/systemd/system/; \
		echo "Systemd service installed. Enable with: sudo systemctl enable virtusdev"; \
	fi
	@echo ""
	@echo "Installation complete!"
	@echo "  Binaries: $(PREFIX)/bin/{virtual_keyboard,virtusdev,keyboard_writer}"
	@echo "  Config: $(SYSCONFDIR)/virtusdev/config"

uninstall:
	rm -f $(PREFIX)/bin/virtual_keyboard
	rm -f $(PREFIX)/bin/virtusdev
	rm -f $(PREFIX)/bin/keyboard_writer
	rm -f /etc/systemd/system/virtusdev.service
	@echo "Uninstalled. Config files preserved in $(SYSCONFDIR)/virtusdev/"

help:
	@echo "VirtUSDev - Virtual Device Emulator - Build Targets:"
	@echo ""
	@echo "  make               - Build all binaries"
	@echo "  make clean         - Remove compiled binaries"
	@echo "  make install       - Install to system (requires root)"
	@echo "  make uninstall     - Remove from system (requires root)"
	@echo "  make help          - Show this help message"
	@echo ""
	@echo "Usage:"
	@echo "  sudo ./virtual_keyboard [device_type]    # Start device emulator"
	@echo "  sudo ./virtusdev [/dev/input/eventX]     # Send barcode data"
	@echo ""
	@echo "For more information:"
	@echo "  README.md          - General usage"
	@echo "  TESTING.md         - Testing procedures"
	@echo ""

.PHONY: all check clean install uninstall help
