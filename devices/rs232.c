#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <linux/input.h>
#include "rs232.h"
#include "../device_config.h"

// ANSI color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_CYAN    "\033[36m"

int rs232_init(int fd) {
    int i;
    
    ioctl(fd, UI_SET_EVBIT, EV_KEY);
    
    for (i = 0; i < 256; i++) {
        ioctl(fd, UI_SET_KEYBIT, i);
    }
    
    ioctl(fd, UI_SET_EVBIT, EV_SYN);
    
    return 0;
}

void rs232_cleanup(int fd) {
    (void)fd;
}

void rs232_print_info(void) {
    printf("%s╔═══════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║      VIRTUAL RS232 SERIAL PORT DEVICE CREATED         ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚═══════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("  %sDevice Name%s  : Virtual RS232 Serial Port\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sDevice Type%s  : RS232 Serial Port Emulator\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %sBaudrate%s     : %d bps\n", COLOR_YELLOW, COLOR_RESET, BAUDRATE);
    printf("  %sVendor ID%s    : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, VENDOR_ID);
    printf("  %sProduct ID%s   : 0x%04X\n", COLOR_YELLOW, COLOR_RESET, PRODUCT_ID);
    printf("%s╔════════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║                                       by @jvwaldrich0  ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚════════════════════════════════════════════════════════╝%s\n", COLOR_CYAN, COLOR_RESET);
    printf("\n%s[INFO]%s Check device location with:\n", COLOR_BLUE, COLOR_RESET);
    printf("       cat /proc/bus/input/devices | grep -A 5 'Virtual RS232'\n");
    printf("\n%s[INFO]%s Press Ctrl+C to destroy the device\n", COLOR_BLUE, COLOR_RESET);
    printf("\n%s[STATUS]%s Device is ready...\n\n", COLOR_GREEN, COLOR_RESET);
}

device_t rs232_device = {
    .name = "rs232",
    .description = "RS232 Serial Port - Virtual serial port communication interface",
    .init = rs232_init,
    .cleanup = rs232_cleanup,
    .print_info = rs232_print_info
};
