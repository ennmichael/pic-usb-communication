#include "led.h"

#include <xc.h>

void led_hardware_setup() {
    TRISDbits.TRISD1 = 0;
    PORTDbits.RD1 = 0;
}

void led_error() { PORTDbits.RD1 = 1; }

