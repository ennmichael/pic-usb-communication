#include "device.h"

#include <xc.h>

#include "ide_helper.h"
#include "led.h"

/* A "store sequence" is when the host wants to store data in the EEPROM of this
 * device. It is initiated by the host writing a single byte designating the
 * length of data, followed by that many bytes of actual data.
 *
 * A "load sequence" is when the host wants to read data from the EEPROM of this
 * device. It is initiated by the host writing a single zero byte, and then
 * reading successive bytes from the device. The first returned byte will be the
 * length of stored data, and the remaining bytes will be the actual data.
 */

typedef enum {
    IDLE,
    IN_STORE_SEQUENCE,  // Host is storing data in the device
    IN_LOAD_SEQUENCE,   // Host is loading data from the device
} DeviceState;

static DeviceState device_state = IDLE;
static uint8_t data[257];

void device_receive(uint8_t byte) {
    static uint8_t receive_counter;

    switch (device_state) {
        case IDLE:
            if (byte == 0) {
                device_state = IN_LOAD_SEQUENCE;
            } else {
                device_state = IN_STORE_SEQUENCE;
                data[0] = byte;
                receive_counter = 1;
            }
            break;
        case IN_STORE_SEQUENCE:
            data[receive_counter] = byte;
            if (receive_counter == data[0]) {
                device_state = IDLE;
            } else {
                receive_counter++;
            }
            break;
        case IN_LOAD_SEQUENCE:
            led_error();
            for (;;) {
            }
            break;
    }
}

uint8_t device_transmit() {
    static uint8_t transmit_counter = 0;
    uint8_t result = 0;

    switch (device_state) {
        case IN_LOAD_SEQUENCE:
            result = data[transmit_counter];
            if (transmit_counter == data[0]) {
                transmit_counter = 0;
                device_state = IDLE;
            } else {
                transmit_counter++;
            }
            return result;
        default:
            led_error();
            for (;;) {
            }
            return 0xFE;
    }
}
