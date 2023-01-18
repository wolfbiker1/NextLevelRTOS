#include "devices/uart.h"
#include "data/list.h"
#include "memory.h"
#include "exception.h"
#include "process/scheduler.h"
#include "panic.h"

UartStates_t state;
TaskInformation_t *tInfo = NULL;
List_t *transfer_list = NULL;

unsigned int in_buffer;
unsigned int byte_in_id;
unsigned int bufferIndex;

char buffer[BUFFERSIZE];
char *p;

void init_transfer_handler(void)
{
    transfer_list = (List_t*) new_list();
}

void __attribute__((optimize("O0"))) setup_transfer(char* address, unsigned int length, TransferType_t type)
{
    if (transfer_list->size > MAX_WAITING_TRANSFERS)
        return;

    if (transfer_list->size == 0)
        wakeup_pid(TASK_TRANSFER_HANDLER);

    TransferInfo_t* t = (TransferInfo_t*) allocate(sizeof(TransferInfo_t));
    
    if (!t)
        invoke_panic(OUT_OF_MEMORY);
    
    t->start_adress = address;
    t->length = length;
    t->type = type;
    push(transfer_list, (void*) t);
}

void __attribute__((optimize("O0"))) transfer_handler(void)
{
    SingleLinkedNode_t* node;
    init_transfer_handler();
    while (1)
    {
        if ((node = pop(transfer_list)))
        {
            unsigned int remaining = TX_LENGTH;
            char *m;
            TransferInfo_t* transfer = (TransferInfo_t*) node->data;
            switch (transfer->type)
            {
            case GENERIC:
                break;
            case MEM_ADDRESS:
                m = "AAAABBBB";
                print(m, 8);
                remaining -= 8;
                break;
            case STATISTIC:
                m = "BBBBAAAA";
                print(m, 8);
                remaining -= 8;
                break;
            case RPM:
                m = "CCCCAAAA";
                print(m, 8);
                remaining -= 8;
                break;
            default:
                break;
            }
            print(transfer->start_adress, transfer->length);
            remaining -= transfer->length;

            char zero = '0';
            while (remaining-- != 0)
                print(&zero, 1);

            if (deallocate((unsigned int*) transfer) == 0)
                invoke_panic(MEMORY_DEALLOC_FAILED);
            if (deallocate((unsigned int*) node) == 0)
                invoke_panic(MEMORY_DEALLOC_FAILED);
        }
        else
        {
            block_current_task();
            SV_YIELD_TASK;
        }
    }    
}

void init_isr(void)
{
    byte_in_id = 4;
    state = RX_READY;
    bufferIndex = 0;
    in_buffer = 0;
    for (unsigned int i = 0; i < BUFFERSIZE; i++)
        buffer[i] = 0;
}

void send_number(unsigned int number, char converted[])
{   
    // unsigned int n = number;
    int cnt = 8;
    
    if (number == 0)
    {
        converted[0] = 0 + 0x30;
    }
    else
    {   
        while (number > 0)
        {   
            converted[cnt] = (number % 10) + 0x30;
            number /= 10;
            cnt--;
        }
    }
}

void __attribute__((interrupt)) uart_isr_handler(void)
{
    // __asm (
    //     "MRS r2, PSP\n"
    //     "STMDB r2!, {r4-r11}\n"
    //     "MSR PSP, r2\n"
    // );
    // __asm volatile ("mrs %0, psp" : "=r"(((Tcb_t*) currently_running->data)->sp));

    switch (state)
    {
    case RX_READY:
        unsigned int content = read_data_register();
        in_buffer |= content << ((--byte_in_id) * 8);
        if ((in_buffer & 0xFFFFFF) == MAGIC)
        {
            state = in_buffer >> 24;
            in_buffer = 0;
            byte_in_id = 4;
        }
        if (byte_in_id == 0)
        {
            in_buffer = 0;
            byte_in_id = 4;
        }
        break;
    case PREPARE_TASK_TRANSFER:
        buffer[bufferIndex++] = read_data_register();
        if (bufferIndex == 4)
        {
            tInfo = (TaskInformation_t*) allocate(sizeof(TaskInformation_t));
            if (!tInfo)
                invoke_panic(OUT_OF_MEMORY);
            
            swap(buffer);
            tInfo->task_size = (unsigned int) *((unsigned int*) buffer); 
            tInfo->start_adress = (char*) allocate(tInfo->task_size); 
            
            if (!tInfo->start_adress)
                invoke_panic(OUT_OF_MEMORY);

            // done because of pyserial testing to receive mem adress everytime!
            state = TRANSFER_TASK_BYTES;
            // state = RX_READY;

            bufferIndex = 0;
            char s1[9] = {0,0,0,0,0,0,0,0,0};
            unsigned int start_addr = (unsigned int) tInfo->start_adress;
            send_number(start_addr, s1);

            // @todo: maybe wrong! replaced &s1 by s1
            setup_transfer(s1, 9, MEM_ADDRESS);

            for (unsigned int i = 0; i < BUFFERSIZE; i++)
                buffer[i] = 0;

            p = tInfo->start_adress;
        }
        break;
    case TRANSFER_TASK_BYTES:
        os_memcpy(p++, read_data_register());
        if (++bufferIndex == tInfo->task_size)
        {
            deallocate((unsigned int*) tInfo);
            create_task((void (*)()) tInfo->start_adress, (unsigned int) tInfo->start_adress);
            state = RX_READY;
            bufferIndex = 0;
        }
        break;
    case REQUEST_STATISTIC:
        wakeup_pid(TASK_STATISTIC);
        state = RX_READY;
        unsigned int dummy = read_data_register();
        // *(unsigned int*) Icsr = *(unsigned int*) Icsr | 1 << PendSVSet;
        return;
        // break;
    case ALTER_SPEED:
        buffer[bufferIndex++] = read_data_register();
        if (bufferIndex == 4)
        {
            volatile unsigned int pwm_speed = (buffer[1] - 0x30) * 100 + (buffer[2] - 0x30) * 10 + (buffer[3] - 0x30);
            volatile unsigned int engine_no = (buffer[0] - 0x30);
            bufferIndex = 0;
            state = RX_READY;

            // should be moved into own function soon
            switch (engine_no)
            {
            case 0:
                // ccr1
                WRITE_REGISTER(0x40000434, pwm_speed);
                break;
            case 1:
                WRITE_REGISTER(0x40000438, pwm_speed);
                break;
            case 2:
                WRITE_REGISTER(0x4000043C, pwm_speed);
                break;
            case 3:
                WRITE_REGISTER(0x40000440, pwm_speed);
                break;            
            default:
                break;
            }
        }
        break; 
    case REQUEST_RPM:
        wakeup_pid(TASK_RPM);
        state = RX_READY;
        unsigned int dummy1 = read_data_register();
        // *(unsigned int*) Icsr = *(unsigned int*) Icsr | 1 << PendSVSet;
        return;
        // break;
    default:
        break;
    }    
    // __asm volatile ("MOV R2, %[input_i]":: [input_i] "r" (((Tcb_t*) currently_running->data)->sp));
    // __asm volatile (
    //   "LDMFD r2!, {r4-r11}\n"
    //   "MSR PSP, r2\n"
    // );
}