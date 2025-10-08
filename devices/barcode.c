#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <linux/input.h>
#include "barcode.h"
#include "../device_config.h"

// ANSI color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_CYAN    "\033[36m"

int barcode_init(int fd) {
    int i;
    
    ioctl(fd, UI_SET_EVBIT, EV_KEY);
    
    for (i = 0; i < 256; i++) {
        ioctl(fd, UI_SET_KEYBIT, i);
    }
    
    ioctl(fd, UI_SET_EVBIT, EV_SYN);
    
    return 0;
}

void barcode_cleanup(int fd) {
    (void)fd;
}

void barcode_print_info(void) {
    printf("%s╔═══════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║      VIRTUAL BARCODE READER DEVICE CREATED            ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚═══════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("  %sDevice Name%s  : %s\n", COLOR_YELLOW, COLOR_RESET, DEVICE_NAME);
    printf("  %sDevice Type%s  : Barcode Scanner (HID Keyboard)\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sBaudrate%s     : %d bps\n", COLOR_YELLOW, COLOR_RESET, BAUDRATE);
    printf("  %sBit Time%s     : %.2f μs\n", COLOR_YELLOW, COLOR_RESET, (float)BIT_TIME_US);
    printf("  %sChar Time%s    : %.2f μs\n", COLOR_YELLOW, COLOR_RESET, (float)CHAR_TIME_US);
    printf("  %sVendor ID%s    : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, VENDOR_ID);
    printf("  %sProduct ID%s   : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, PRODUCT_ID);
    printf("  %sDate%s         : 2025-10-08 12:59:34 UTC\n", COLOR_YELLOW, COLOR_RESET);
    printf("%s╔════════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║                                       by @jvwaldrich0  ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚════════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("\n%s[INFO]%s Check device location with:\n", COLOR_BLUE, COLOR_RESET);
    printf("       cat /proc/bus/input/devices | grep -A 5 'Virtual Keyboard 115200'\n");
    printf("\n%s[INFO]%s Press Ctrl+C to destroy the device\n", COLOR_BLUE, COLOR_RESET);
    printf("\n%s[STATUS]%s Device is ready and waiting for barcode scans...\n\n", COLOR_GREEN, COLOR_RESET);
}

device_t barcode_device = {
    .name = "barcode",
    .description = "Barcode Scanner (115200 baud) - Emulates USB HID barcode reader",
    .init = barcode_init,
    .cleanup = barcode_cleanup,
    .print_info = barcode_print_info
};
