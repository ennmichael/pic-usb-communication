#pragma config FOSC = INTOSC
#pragma config WDTE = OFF
#pragma config PWRTE = OFF
#pragma config MCLRE = ON
#pragma config CP = OFF
#pragma config CPD = OFF
#pragma config BOREN = ON
#pragma config CLKOUTEN = OFF
#pragma config IESO = OFF
#pragma config FCMEN = ON

#pragma config WRT = OFF
#pragma config VCAPEN = OFF
#pragma config PLLEN = ON
#pragma config STVREN = ON
#pragma config BORV = LO
#pragma config LVP = ON

#define _XTAL_FREQ 16000000

#include <stdint.h>
#include <xc.h>

#include "device.h"
#include "ide_helper.h"
#include "led.h"

void setup_oscillator(void) { OSCCONbits.IRCF = 0b1111; }

void setup_pins(void) {
    TRISCbits.TRISC3 = 1;
    TRISCbits.TRISC4 = 1;
}

void setup_i2c(void) {
    SSPADD = 0x08;
    SSPCON1bits.SSPM = 0b1110;
    SSPCON1bits.SSPEN = 1;
    SSPCON2bits.SEN = 1;
}

void setup_interrupts(void) {
    INTCONbits.PEIE = 1;
    INTCONbits.GIE = 1;
}

void setup_hardware(void) {
    led_hardware_setup();
    setup_oscillator();
    setup_pins();
    setup_interrupts();
    setup_i2c();
}

static uint8_t write_counter = 0;
static uint8_t read_counter = 0;
static uint8_t test_data[100];

static volatile uint8_t xnop = 0;

void handle_i2c_transmission() {
    if (SSPSTATbits.S == 0) {
        led_error();
        for (;;) {
        }
    }

    SSPIF = 0;
    (void)SSPBUF;

    for (;;) {
        if (!SSPIF) {
            continue;
        }

        SSPIF = 0;
        (void)SSPBUF;  // Clears the BF

        if (SSPSTATbits.P || SSPCON2bits.ACKSTAT) {
            break;
        }

        // SSPBUF = device_transmit();
        SSPBUF = test_data[read_counter];
        read_counter++;

        SSPCON1bits.CKP = 1;
    }
}

static volatile uint8_t counter = 0;

void handle_i2c_reception() {
    if (SSPSTATbits.S == 0) {
        led_error();
        for (;;) {
        }
    }

    SSPIF = 0;
    (void)SSPBUF;

    for (;;) {
        if (!SSPIF) {
            continue;
        }

        SSPIF = 0;
        (void)SSPBUF;  // Clears the BF

        if (SSPSTATbits.P) {
            break;
        }

        if (SSPSTATbits.D_nA) {
            // device_receive(SSPBUF);
            test_data[write_counter] = SSPBUF;
            write_counter++;
        }

        SSPCON1bits.CKP = 1;
    }
}

int main(void) {
    setup_hardware();

    test_data[0] = 0x0F;
    test_data[1] = 0x0E;
    test_data[2] = 0x0D;
    test_data[3] = 0x0C;
    test_data[4] = 0x0B;
    test_data[5] = 0x0A;
    test_data[6] = 0x09;

    for (;;) {
        if (!SSPIF) {
            continue;
        }

        if (SSPSTATbits.R_nW) {
            handle_i2c_transmission();
        } else {
            if (SSPBUF % 2 == 1) {
                handle_i2c_transmission();
            } else {
                handle_i2c_reception();
            }
        }
    }

    return 0;
}
