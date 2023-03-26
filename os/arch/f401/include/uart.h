#ifndef UART_H
#define UART_H
#include "gpio.h"

enum { Usart1Baseadress = 0x40011000 };

#define RCC_USART1EN 4
#define TC 6
#define TXE 7

typedef struct UartRegisterMap {
    unsigned int sr;
    unsigned int dr;
    unsigned int brr;
    unsigned int cr1;
    unsigned int cr2;
    unsigned int cr3;
    unsigned int gtpr;
} UartRegisterMap_t;

void init_uart(GpioObject_t*);

static inline __attribute__((always_inline)) unsigned int read_data_register(void)
{
    return *((unsigned int*) (Usart1Baseadress | 0x04));
}

static inline __attribute__((always_inline)) void print(char* src, unsigned int length)
{
    for (unsigned int i = 0; i < length; i++)
    {
        while (!((READ_REGISTER(&((UartRegisterMap_t*) Usart1Baseadress)->sr) & (1 << TXE)) != 0));
        WRITE_REGISTER(&((UartRegisterMap_t*) Usart1Baseadress)->dr,*src);
        src++;
    }
    while (!(( READ_REGISTER(&((UartRegisterMap_t*) Usart1Baseadress)->sr) & (1 << TC)) != 0));
}

#endif