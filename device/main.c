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

static void oscillator_hardware_setup(void) { OSCCONbits.IRCF = 0b1111; }

static void pins_hardware_setup(void) {
    TRISCbits.TRISC3 = 1;
    TRISCbits.TRISC4 = 1;
}

static void i2c_hardware_setup(void) {
    SSPADD = 0x08;
    SSPCON1bits.SSPM = 0b0110;
    SSPCON1bits.SSPEN = 1;
    SSPCON2bits.SEN = 1;
    SSPCON3bits.BOEN = 1;
}

static void interrupts_hardware_setup(void) {
    INTCONbits.PEIE = 1;
    INTCONbits.GIE = 1;
}

static void hardware_setup(void) {
    led_hardware_setup();
    oscillator_hardware_setup();
    pins_hardware_setup();
    interrupts_hardware_setup();
    i2c_hardware_setup();
}

static void main_loop() {
    uint8_t address_received = 0;

    for (;;) {
        if (SSPSTATbits.P) {
            address_received = 0;
        }

        if (!PIR1bits.SSPIF) {
            continue;
        }

        PIR1bits.SSPIF = 0;
        (void)SSPBUF;  // Clears BF

        if (SSPSTATbits.D_nA && SSPCON2bits.ACKSTAT) {
            continue;
        }

        if (SSPSTATbits.R_nW) {
            SSPBUF = device_transmit();
        } else {
            if (address_received) {
                device_receive(SSPBUF);
            }
            address_received = 1;
        }

        SSPCON1bits.CKP = 1;
    }
}

int main(void) {
    hardware_setup();
    main_loop();
    return 0;
}
