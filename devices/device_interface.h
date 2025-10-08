#ifndef DEVICE_INTERFACE_H
#define DEVICE_INTERFACE_H

#include <linux/uinput.h>

typedef struct {
    const char *name;
    const char *description;
    int (*init)(int fd);
    void (*cleanup)(int fd);
    void (*print_info)(void);
} device_t;

// Device implementations
extern device_t barcode_device;
extern device_t usb_keyboard_device;
extern device_t mouse_jiggle_device;
extern device_t rs232_device;

#endif
