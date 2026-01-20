CC = gcc
CFLAGS = -Wall -Wextra -O2
TARGETS = virtual_keyboard keyboard_writer

all: $(TARGETS)

virtual_keyboard: virtual_keyboard.c device_config.h
	$(CC) $(CFLAGS) -o virtual_keyboard virtual_keyboard.c

keyboard_writer: keyboard_writer.c device_config.h
	$(CC) $(CFLAGS) -o keyboard_writer keyboard_writer.c

check: $(TARGETS)
	@echo "Running basic checks..."
	@echo "[1/3] Checking virtual_keyboard binary..."
	@test -f virtual_keyboard || (echo "ERROR: virtual_keyboard not built" && exit 1)
	@echo "[2/3] Checking keyboard_writer binary..."
	@test -f keyboard_writer || (echo "ERROR: keyboard_writer not built" && exit 1)
	@echo "[3/3] Checking file permissions..."
	@test -x virtual_keyboard || (echo "ERROR: virtual_keyboard not executable" && exit 1)
	@test -x keyboard_writer || (echo "ERROR: keyboard_writer not executable" && exit 1)
	@echo "✅ All checks passed!"

clean:
	rm -f $(TARGETS)

install: all
	@echo "Run with sudo:"
	@echo "  sudo ./virtual_keyboard    # Terminal 1"
	@echo "  sudo ./keyboard_writer     # Terminal 2"

help:
	@echo "Virtual Barcode Reader - Build Targets:"
	@echo ""
	@echo "  make               - Build all binaries"
	@echo "  make clean         - Remove compiled binaries"
	@echo "  make install       - Show installation instructions"
	@echo "  make help          - Show this help message"
	@echo ""
	@echo "For more information:"
	@echo "  README.md          - General usage"
	@echo "  TESTING.md         - Testing procedures"
	@echo ""

.PHONY: all check clean install help
