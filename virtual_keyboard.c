#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <signal.h>
#include "device_config.h"

static int fd = -1;

void cleanup(int signo) {
    if (fd >= 0) {
        ioctl(fd, UI_DEV_DESTROY);
        close(fd);
    }
    printf("\n[EXIT] Virtual barcode reader device destroyed\n");
    exit(0);
}

void print_device_info(void) {
    printf("╔═══════════════════════════════════════════════════════╗\n");
    printf("║      VIRTUAL BARCODE READER DEVICE CREATED            ║\n");
    printf("╚═══════════════════════════════════════════════════════╝\n");
    printf("  Device Name  : %s\n", DEVICE_NAME);
    printf("  Device Type  : Barcode Scanner (HID Keyboard)\n");
    printf("  Baudrate     : %d bps\n", BAUDRATE);
    printf("  Bit Time     : %.2f μs\n", (float)BIT_TIME_US);
    printf("  Char Time    : %.2f μs\n", (float)CHAR_TIME_US);
    printf("  Vendor ID    : 0x%04X\n", VENDOR_ID);
    printf("  Product ID   : 0x%04X\n", PRODUCT_ID);
    printf("  Date         : 2025-10-08 12:59:34 UTC\n");
    printf("╔════════════════════════════════════════════════════════╗\n");
    printf("║                                       by @jvwaldrich0  ║\n");
    printf("╚════════════════════════════════════════════════════════╝\n");
    printf("\n[INFO] Check device location with:\n");
    printf("       cat /proc/bus/input/devices | grep -A 5 'Virtual Keyboard 115200'\n");
    printf("\n[INFO] Press Ctrl+C to destroy the device\n");
    printf("\n[STATUS] Device is ready and waiting for barcode scans...\n\n");
}

int main(void) {
    struct uinput_setup usetup;
    int i;

    signal(SIGINT, cleanup);
    signal(SIGTERM, cleanup);

    fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
    if (fd < 0) {
        perror("[ERROR] Cannot open /dev/uinput");
        printf("[FIX] Try: sudo modprobe uinput\n");
        exit(1);
    }

    ioctl(fd, UI_SET_EVBIT, EV_KEY);
    
    for (i = 0; i < 256; i++) {
        ioctl(fd, UI_SET_KEYBIT, i);
    }

    ioctl(fd, UI_SET_EVBIT, EV_SYN);

    memset(&usetup, 0, sizeof(usetup));
    usetup.id.bustype = BUS_USB;
    usetup.id.vendor = VENDOR_ID;
    usetup.id.product = PRODUCT_ID;
    usetup.id.version = DEVICE_VERSION;
    strcpy(usetup.name, DEVICE_NAME);

    ioctl(fd, UI_DEV_SETUP, &usetup);
    
    if (ioctl(fd, UI_DEV_CREATE) < 0) {
        perror("[ERROR] UI_DEV_CREATE failed");
        close(fd);
        exit(1);
    }

    usleep(100000);

    print_device_info();

    while (1) {
        sleep(1);
    }

    return 0;
}