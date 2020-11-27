/*
#ifdef NDEBUG
#pragma config DEBUG = OFF
#else
#pragma config DEBUG = ON
#endif//DEBUG
 */

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

#include "ide_helper.h"

void oscillatorSetup(void) { OSCCONbits.IRCF = 0b1111; }

void pinsSetup(void) {
    TRISCbits.TRISC3 = 1;
    TRISCbits.TRISC4 = 1;
    TRISDbits.TRISD1 = 0;
}

void i2cSetup(void) {
    SSPADD = 0x08;
    SSPCON2bits.GCEN = 1;
    SSPCON1bits.SSPM = 0b0110;  // I2C Slave, 7 bits
    SSPCON1bits.SSPEN = 1;
    SSPCON3bits.AHEN = 0;
    SSPCON3bits.DHEN = 0;
}

void interruptsSetup(void) {
    PIE1bits.SSPIE = 1;
    INTCONbits.PEIE = 1;
    INTCONbits.GIE = 1;
}

void hardwareSetup(void) {
    oscillatorSetup();
    pinsSetup();
    interruptsSetup();
    i2cSetup();
}

uint8_t sspif = 0;
uint8_t buf = 0;

void __interrupt() interruptHandler(void) {
    sspif = 1;
    buf = SSPBUF;
    PORTDbits.RD1 = 1;
}

int main(void) {
    hardwareSetup();
    PORTDbits.RD1 = 0;
    for (;;) {
        sspif = PIR1bits.SSPIF;
    }
    return 0;
}
