#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <linux/input.h>
#include "usb_keyboard.h"
#include "../device_config.h"

// ANSI color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_CYAN    "\033[36m"

int usb_keyboard_init(int fd) {
    int i;
    
    ioctl(fd, UI_SET_EVBIT, EV_KEY);
    ioctl(fd, UI_SET_EVBIT, EV_LED);
    
    for (i = 0; i < 256; i++) {
        ioctl(fd, UI_SET_KEYBIT, i);
    }
    
    ioctl(fd, UI_SET_EVBIT, EV_SYN);
    
    return 0;
}

void usb_keyboard_cleanup(int fd) {
    (void)fd;
}

void usb_keyboard_print_info(void) {
    printf("%s╔═══════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║        VIRTUAL USB KEYBOARD DEVICE CREATED            ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚═══════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("  %sDevice Name%s  : Virtual USB Keyboard\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sDevice Type%s  : USB HID Keyboard\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sVendor ID%s    : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, VENDOR_ID);
    printf("  %sProduct ID%s   : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, PRODUCT_ID);
    printf("%s╔════════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║                                       by @jvwaldrich0  ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚════════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("\n%s[INFO]%s Check device location with:\n", COLOR_BLUE, COLOR_RESET);
    printf("       cat /proc/bus/input/devices | grep -A 5 'Virtual USB Keyboard'\n");
    printf("\n%s[INFO]%s Press Ctrl+C to destroy the device\n", COLOR_BLUE, COLOR_RESET);
    printf("\n%s[STATUS]%s Device is ready...\n\n", COLOR_GREEN, COLOR_RESET);
}

device_t usb_keyboard_device = {
    .name = "usb_keyboard",
    .description = "USB Keyboard - Full featured virtual USB HID keyboard",
    .init = usb_keyboard_init,
    .cleanup = usb_keyboard_cleanup,
    .print_info = usb_keyboard_print_info
};
