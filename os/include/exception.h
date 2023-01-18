#ifndef EXCEPTION_H
#define EXCEPTION_H
#include <stdint.h>
#include "lang.h"

#ifdef SELF_CHECK
    // do_selfcheck_memory();
#endif

#define svc(code) asm volatile ("mov r6, %[immediate]\n" \
                                " SVC \#0"::[immediate] "I" (code) \
                                 : "r6")


#define SV_EXEC_PSP_TASK __asm volatile ("mov r6, 0\n" \
                                  "svc 0\n")

#define SV_YIELD_TASK __asm volatile ("mov r6, 2\n" \
                                  "svc 0\n")

#define SV_STD __asm volatile ("mov r6, 3\n" \
                                  "svc 0\n")

#define SV_STE __asm volatile ("mov r6, 4\n" \
                                  "svc 0\n")

#define SV_PRINT __asm volatile ("mov r6, 1\n" \
                                  "mov r9, r0\n"\
                                  "svc 0\n")
                                  
                                  
__attribute__((used)) void uprint(volatile unsigned int* transfer_info, volatile unsigned int type);

typedef enum {
    RET_PSP_THREAD_NOFP = 0xFFFFFFFD,
    RET_MSP_THREAD_NOFP = 0xFFFFFFF9,
} ExcReturn_t;

typedef enum {
    EXEC_PSP_TASK = 0,
    PRINT_MSG,
    YIELD_TASK,
    STD,
    STE,
    RESET
} TrapType_t;

void do_selfcheck_svc();

#endif