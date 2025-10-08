#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <signal.h>
#include "device_config.h"
#include "devices/device_interface.h"
#include "devices/barcode.h"
#include "devices/usb_keyboard.h"
#include "devices/mouse_jiggle.h"
#include "devices/rs232.h"

// ANSI color codes
#define COLOR_RESET   "\033[0m"
#define COLOR_RED     "\033[31m"
#define COLOR_GREEN   "\033[32m"
#define COLOR_YELLOW  "\033[33m"
#define COLOR_BLUE    "\033[34m"
#define COLOR_MAGENTA "\033[35m"
#define COLOR_CYAN    "\033[36m"

static int fd = -1;
static device_t *current_device = NULL;

void cleanup(int signo) {
    (void)signo;
    if (fd >= 0) {
        if (current_device && current_device->cleanup) {
            current_device->cleanup(fd);
        }
        ioctl(fd, UI_DEV_DESTROY);
        close(fd);
    }
    printf("\n%s[EXIT]%s Virtual device destroyed\n", COLOR_YELLOW, COLOR_RESET);
    exit(0);
}


device_t* get_devices(int *count) {
    static device_t devices[] = {
        {0},  // Will be filled with barcode_device
        {0},  // Will be filled with usb_keyboard_device
        {0},  // Will be filled with mouse_jiggle_device
        {0}   // Will be filled with rs232_device
    };
    static int initialized = 0;
    
    if (!initialized) {
        devices[0] = barcode_device;
        devices[1] = usb_keyboard_device;
        devices[2] = mouse_jiggle_device;
        devices[3] = rs232_device;
        initialized = 1;
    }
    
    *count = 4;
    return devices;
}

void print_usage(const char *prog) {
    int i, count;
    device_t *devices = get_devices(&count);
    
    printf("%sUsage:%s %s [DEVICE_TYPE]\n\n", COLOR_CYAN, COLOR_RESET, prog);
    printf("%sAvailable devices:%s\n", COLOR_YELLOW, COLOR_RESET);
    for (i = 0; i < count; i++) {
        printf("  %s%-15s%s - %s\n", COLOR_GREEN, devices[i].name, COLOR_RESET, devices[i].description);
    }
    printf("\n%sExamples:%s\n", COLOR_YELLOW, COLOR_RESET);
    printf("  %s barcode        %s# Start barcode scanner device\n", prog, COLOR_RESET);
    printf("  %s usb_keyboard   %s# Start USB keyboard device\n", prog, COLOR_RESET);
    printf("  %s               %s# Interactive menu\n\n", prog, COLOR_RESET);
}

device_t* select_device_interactive() {
    int i, count, choice;
    device_t *devices = get_devices(&count);
    
    printf("\n%s╔═══════════════════════════════════════════════════════╗%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s║           VirtUSDev - Device Selection Menu           ║%s\n", COLOR_CYAN, COLOR_RESET);
    printf("%s╚═══════════════════════════════════════════════════════╝%s\n\n", COLOR_CYAN, COLOR_RESET);
    
    printf("%sSelect a device to emulate:%s\n\n", COLOR_YELLOW, COLOR_RESET);
    
    for (i = 0; i < count; i++) {
        printf("  %s[%d]%s %s%-15s%s\n", COLOR_CYAN, i + 1, COLOR_RESET, COLOR_GREEN, devices[i].name, COLOR_RESET);
        printf("      %s\n\n", devices[i].description);
    }
    
    printf("%sEnter choice [1-%d]: %s", COLOR_YELLOW, count, COLOR_RESET);
    fflush(stdout);
    
    if (scanf("%d", &choice) != 1 || choice < 1 || choice > count) {
        printf("%s[ERROR]%s Invalid choice\n", COLOR_RED, COLOR_RESET);
        return NULL;
    }
    
    return &devices[choice - 1];
}

device_t* find_device_by_name(const char *name) {
    int i, count;
    device_t *devices = get_devices(&count);
    
    for (i = 0; i < count; i++) {
        if (strcmp(devices[i].name, name) == 0) {
            return &devices[i];
        }
    }
    
    return NULL;
}

char* read_config_file(const char *config_path) {
    FILE *fp;
    static char device_name[64];
    char line[256];
    
    fp = fopen(config_path, "r");
    if (!fp) {
        return NULL;
    }
    
    while (fgets(line, sizeof(line), fp)) {
        // Skip comments and empty lines
        if (line[0] == '#' || line[0] == '\n') {
            continue;
        }
        
        // Look for DEVICE= line
        if (strncmp(line, "DEVICE=", 7) == 0) {
            char *value = line + 7;
            // Remove trailing newline
            value[strcspn(value, "\n")] = 0;
            // Remove quotes if present
            if (value[0] == '"' || value[0] == '\'') {
                value++;
                value[strlen(value) - 1] = 0;
            }
            strncpy(device_name, value, sizeof(device_name) - 1);
            device_name[sizeof(device_name) - 1] = 0;
            fclose(fp);
            return device_name;
        }
    }
    
    fclose(fp);
    return NULL;
}

int main(int argc, char *argv[]) {
    struct uinput_setup usetup;
    device_t *selected_device = NULL;
    
    signal(SIGINT, cleanup);
    signal(SIGTERM, cleanup);
    
    // Try to read from config file first
    char *config_device = read_config_file("/etc/virtusdev/config");
    if (config_device) {
        printf("%s[CONFIG]%s Using device from /etc/virtusdev/config: %s\n", 
               COLOR_BLUE, COLOR_RESET, config_device);
        selected_device = find_device_by_name(config_device);
        if (!selected_device) {
            printf("%s[WARNING]%s Device '%s' from config not found, ignoring\n", 
                   COLOR_YELLOW, COLOR_RESET, config_device);
        }
    }
    
    // Command line argument overrides config file
    if (argc >= 2) {
        if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
            print_usage(argv[0]);
            return 0;
        }
        
        selected_device = find_device_by_name(argv[1]);
        if (!selected_device) {
            printf("%s[ERROR]%s Unknown device type: %s\n", COLOR_RED, COLOR_RESET, argv[1]);
            print_usage(argv[0]);
            return 1;
        }
    }
    
    // If no device selected yet, show interactive menu
    if (!selected_device) {
        selected_device = select_device_interactive();
        if (!selected_device) {
            return 1;
        }
    }
    
    current_device = selected_device;
    
    printf("\n%s[INIT]%s Initializing %s%s%s device...\n", 
           COLOR_BLUE, COLOR_RESET, COLOR_GREEN, selected_device->name, COLOR_RESET);
    
    fd = open("/dev/uinput", O_WRONLY | O_NONBLOCK);
    if (fd < 0) {
        perror("[ERROR] Cannot open /dev/uinput");
        printf("%s[FIX]%s Try: sudo modprobe uinput\n", COLOR_YELLOW, COLOR_RESET);
        exit(1);
    }
    
    // Initialize device-specific setup
    if (selected_device->init) {
        selected_device->init(fd);
    }

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

    if (selected_device->print_info) {
        selected_device->print_info();
    }

    while (1) {
        sleep(1);
    }

    return 0;
}