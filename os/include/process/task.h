#ifndef TASK_H
#define TASK_H
#include "hw/cpu.h"

#define THUMB_STATE 1 << 24

typedef struct {
    unsigned int task_size;
    char* start_adress;
} TaskInformation_t;

void create_task(void (*)(), unsigned int);
void do_selfcheck_task(void);
CpuRegister_t* prepare_cpu_register(unsigned int, unsigned int, void (*)());

#endif