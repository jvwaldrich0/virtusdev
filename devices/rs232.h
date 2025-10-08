#ifndef RS232_DEVICE_H
#define RS232_DEVICE_H

#include "device_interface.h"

int rs232_init(int fd);
void rs232_cleanup(int fd);
void rs232_print_info(void);

#endif
