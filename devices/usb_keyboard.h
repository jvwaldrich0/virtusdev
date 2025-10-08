#ifndef USB_KEYBOARD_DEVICE_H
#define USB_KEYBOARD_DEVICE_H

#include "device_interface.h"

int usb_keyboard_init(int fd);
void usb_keyboard_cleanup(int fd);
void usb_keyboard_print_info(void);

#endif
