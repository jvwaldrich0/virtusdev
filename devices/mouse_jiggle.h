#ifndef MOUSE_JIGGLE_DEVICE_H
#define MOUSE_JIGGLE_DEVICE_H

#include "device_interface.h"

int mouse_jiggle_init(int fd);
void mouse_jiggle_cleanup(int fd);
void mouse_jiggle_print_info(void);

#endif
