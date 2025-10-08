#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <linux/input.h>
#include "mouse_jiggle.h"
#include "../device_config.h"

// ANSI color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_CYAN    "\033[36m"

int mouse_jiggle_init(int fd) {
    ioctl(fd, UI_SET_EVBIT, EV_KEY);
    ioctl(fd, UI_SET_EVBIT, EV_REL);
    
    ioctl(fd, UI_SET_RELBIT, REL_X);
    ioctl(fd, UI_SET_RELBIT, REL_Y);
    
    ioctl(fd, UI_SET_KEYBIT, BTN_LEFT);
    ioctl(fd, UI_SET_KEYBIT, BTN_RIGHT);
    ioctl(fd, UI_SET_KEYBIT, BTN_MIDDLE);
    
    return 0;
}

void mouse_jiggle_cleanup(int fd) {
    (void)fd;
}

void mouse_jiggle_print_info(void) {
    printf("%s╔═══════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║        VIRTUAL MOUSE JIGGLER DEVICE CREATED           ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚═══════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("  %sDevice Name%s  : Virtual Mouse Jiggler\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sDevice Type%s  : USB HID Mouse (Auto-jiggle)\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sVendor ID%s    : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, VENDOR_ID);
    printf("  %sProduct ID%s   : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, PRODUCT_ID);
    printf("%s╔════════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║                                       by @jvwaldrich0  ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚════════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("\n%s[INFO]%s Check device location with:\n", COLOR_BLUE, COLOR_RESET);
    printf("       cat /proc/bus/input/devices | grep -A 5 'Virtual Mouse Jiggler'\n");
    printf("\n%s[INFO]%s Press Ctrl+C to destroy the device\n", COLOR_BLUE, COLOR_RESET);
    printf("\n%s[STATUS]%s Device is ready (jiggling to prevent sleep)...\n\n", COLOR_GREEN, COLOR_RESET);
}

device_t mouse_jiggle_device = {
    .name = "mouse_jiggle",
    .description = "Mouse Jiggler - Prevents screen lock by moving mouse periodically",
    .init = mouse_jiggle_init,
    .cleanup = mouse_jiggle_cleanup,
    .print_info = mouse_jiggle_print_info
};
