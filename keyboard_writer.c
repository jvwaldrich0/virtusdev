#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/uinput.h>
#include <linux/input.h>
#include <ctype.h>
#include <time.h>
#include "device_config.h"

int get_keycode(char c) {
    if (c >= 'a' && c <= 'z') return KEY_A + (c - 'a');
    if (c >= 'A' && c <= 'Z') return KEY_A + (c - 'A');
    if (c >= '0' && c <= '9') return KEY_1 + (c - '1');
    
    switch (c) {
        case ' ': return KEY_SPACE;
        case '\n': return KEY_ENTER;
        case '\t': return KEY_TAB;
        case '-': return KEY_MINUS;
        case '=': return KEY_EQUAL;
        case '[': return KEY_LEFTBRACE;
        case ']': return KEY_RIGHTBRACE;
        case ';': return KEY_SEMICOLON;
        case '\'': return KEY_APOSTROPHE;
        case '`': return KEY_GRAVE;
        case '\\': return KEY_BACKSLASH;
        case ',': return KEY_COMMA;
        case '.': return KEY_DOT;
        case '/': return KEY_SLASH;
        case '_': return KEY_MINUS;  // With shift
        case '+': return KEY_EQUAL;  // With shift
        default: return -1;
    }
}

int needs_shift(char c) {
    if (c >= 'A' && c <= 'Z') return 1;
    if (strchr("!@#$%^&*()_+{}|:\"<>?~", c)) return 1;
    return 0;
}

void emit(int fd, int type, int code, int val) {
    struct input_event ev;
    memset(&ev, 0, sizeof(ev));
    
    gettimeofday(&ev.time, NULL);
    ev.type = type;
    ev.code = code;
    ev.value = val;
    
    if (write(fd, &ev, sizeof(ev)) < 0) {
        perror("Error writing event");
    }
}

void send_key(int fd, char c) {
    int keycode = get_keycode(tolower(c));
    
    if (keycode < 0) {
        printf("Warning: No keycode for character '%c' (0x%02X)\n", c, (unsigned char)c);
        return;
    }
    
    if (needs_shift(c)) {
        emit(fd, EV_KEY, KEY_LEFTSHIFT, 1);
        emit(fd, EV_SYN, SYN_REPORT, 0);
        usleep(KEY_PRESS_DELAY);
    }
    
    emit(fd, EV_KEY, keycode, 1);
    emit(fd, EV_SYN, SYN_REPORT, 0);
    usleep(KEY_PRESS_DELAY);
    
    emit(fd, EV_KEY, keycode, 0);
    emit(fd, EV_SYN, SYN_REPORT, 0);
    usleep(KEY_RELEASE_DELAY);
    
    if (needs_shift(c)) {
        emit(fd, EV_KEY, KEY_LEFTSHIFT, 0);
        emit(fd, EV_SYN, SYN_REPORT, 0);
        usleep(KEY_RELEASE_DELAY);
    }
    
    usleep(INTER_KEY_DELAY);
}

void send_barcode(int fd, const char *barcode) {
    int len = strlen(barcode);
    
    printf("[BARCODE SCAN] Sending: %s\n", barcode);
    printf("[BARCODE SCAN] Length: %d characters at %d baud\n", len, BAUDRATE);
    
    struct timespec start, end;
    clock_gettime(CLOCK_MONOTONIC, &start);
    
    for (int i = 0; barcode[i] != '\0'; i++) {
        send_key(fd, barcode[i]);
    }
    
    send_key(fd, '\n');
    
    clock_gettime(CLOCK_MONOTONIC, &end);
    
    long duration_us = (end.tv_sec - start.tv_sec) * 1000000 + 
                      (end.tv_nsec - start.tv_nsec) / 1000;
    
    printf("[BARCODE SCAN] Complete! (took %ld μs, %.2f ms)\n", duration_us, duration_us / 1000.0);
    printf("[BARCODE SCAN] Effective rate: %.0f chars/sec\n\n", 
           (len + 1) * 1000000.0 / duration_us);
}

int find_virtual_keyboard() {
    char line[64];
    char device_path[512];
    FILE *fp;
    
    char cmd[512];
    snprintf(cmd, sizeof(cmd), 
             "cat /proc/bus/input/devices | grep -A 5 '%s' | grep -o 'event[0-9]*' | tail -1",
             DEVICE_NAME);
    
    fp = popen(cmd, "r");
    if (fp == NULL) {
        return -1;
    }
    
    if (fgets(line, sizeof(line), fp) != NULL) {
        line[strcspn(line, "\n")] = 0;
        
        if (strncmp(line, "event", 5) == 0) {
            snprintf(device_path, sizeof(device_path), "/dev/input/%s", line);
            pclose(fp);
            
            printf("[DEVICE] Found virtual keyboard at: %s\n", device_path);
            return open(device_path, O_WRONLY | O_NONBLOCK);
        }
    }
    
    pclose(fp);
    return -1;
}

void print_header(void) {
    printf("╔═══════════════════════════════════════════════════════╗\n");
    printf("║         BARCODE READER EMULATOR (115200 baud)        ║\n");
    printf("╚═══════════════════════════════════════════════════════╝\n");
    printf("  Device     : %s\n", DEVICE_NAME);
    printf("  Baudrate   : %d bps\n", BAUDRATE);
    printf("  Char Time  : %d μs\n", CHAR_TIME_US);
    printf("  User       : jvwaldrich0\n");
    printf("  Date       : 2025-10-08 12:59:34 UTC\n");
    printf("══════════════════════════════════════════════════════════\n\n");
}

void print_usage(const char *prog) {
    printf("Usage:\n");
    printf("  Interactive mode:\n");
    printf("    %s [/dev/input/eventX]\n\n", prog);
    printf("  Direct barcode input:\n");
    printf("    echo 'barcode_data' | %s [/dev/input/eventX]\n\n", prog);
    printf("  Single barcode:\n");
    printf("    %s [/dev/input/eventX] 'barcode_data'\n\n", prog);
    printf("Examples:\n");
    printf("  %s /dev/input/event25\n", prog);
    printf("  echo '1234567890' | %s /dev/input/event25\n", prog);
    printf("  %s /dev/input/event25 'ABC123XYZ'\n", prog);
}

int main(int argc, char *argv[]) {
    int fd;
    char input[1024];
    char *device_arg = NULL;
    char *barcode_arg = NULL;
    
    print_header();
    
    if (argc >= 2) {
        if (strncmp(argv[1], "/dev/", 5) == 0) {
            device_arg = argv[1];
            if (argc >= 3) {
                barcode_arg = argv[2];
            }
        } else if (strcmp(argv[1], "-h") == 0 || strcmp(argv[1], "--help") == 0) {
            print_usage(argv[0]);
            return 0;
        } else {
            barcode_arg = argv[1];
        }
    }
    
    if (device_arg) {
        fd = open(device_arg, O_WRONLY | O_NONBLOCK);
        if (fd < 0) {
            perror("[ERROR] Cannot open device");
            return 1;
        }
        printf("[DEVICE] Using: %s\n\n", device_arg);
    } else {
        fd = find_virtual_keyboard();
        if (fd < 0) {
            printf("[ERROR] Cannot find virtual keyboard device\n");
            printf("[ERROR] Make sure virtual_keyboard is running!\n\n");
            print_usage(argv[0]);
            return 1;
        }
        printf("\n");
    }
    
    if (barcode_arg) {
        send_barcode(fd, barcode_arg);
        close(fd);
        return 0;
    }
    
    int is_interactive = isatty(fileno(stdin));
    
    if (is_interactive) {
        printf("[MODE] Interactive barcode scanner mode\n");
        printf("[INFO] Enter barcode data and press Enter (Ctrl+D to exit)\n");
        printf("[INFO] Each line will be sent as a complete barcode scan\n\n");
    } else {
        printf("[MODE] Pipe/redirect mode - reading from stdin\n\n");
    }
    
    while (1) {
        if (is_interactive) {
            printf("SCAN> ");
            fflush(stdout);
        }
        
        if (fgets(input, sizeof(input), stdin) == NULL) {
            break;
        }
        
        input[strcspn(input, "\n")] = 0;
        
        if (strlen(input) == 0) {
            continue;
        }
        
        send_barcode(fd, input);
    }
    
    close(fd);
    printf("\n[EXIT] Barcode reader emulator stopped\n");
    return 0;
}