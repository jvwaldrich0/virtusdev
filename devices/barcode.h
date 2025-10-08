#ifndef BARCODE_DEVICE_H
#define BARCODE_DEVICE_H

#include "device_interface.h"

int barcode_init(int fd);
void barcode_cleanup(int fd);
void barcode_print_info(void);

#endif
