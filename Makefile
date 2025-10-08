CC = gcc
CFLAGS = -Wall -Wextra -O2
TARGETS = virtual_keyboard keyboard_writer

all: $(TARGETS)

virtual_keyboard: virtual_keyboard.c device_config.h
	$(CC) $(CFLAGS) -o virtual_keyboard virtual_keyboard.c

keyboard_writer: keyboard_writer.c device_config.h
	$(CC) $(CFLAGS) -o keyboard_writer keyboard_writer.c

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

.PHONY: all clean install help