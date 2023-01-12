#include "devices/uart.h"
#include "data/list.h"
#include "memory.h"
#include "exception.h"
#include "process/scheduler.h"
#include "panic.h"

UartStates_t state;
TaskInformation_t *tInfo = NULL;

List_t *transfer_list = NULL;
unsigned int in_buffer = 0;
unsigned int byte_in_id;
unsigned int bufferIndex;

#define BUFFERSIZE 4
#define TX_LENGTH 64
char buffer[BUFFERSIZE];
char *p;

void init_transfer_handler(void)
{
    transfer_list = (List_t*) new_list();
}

void __attribute__((optimize("O0"))) setup_transfer(char* address, unsigned int length, TransferType_t type)
{
    if (transfer_list->size > 10)
        return;

    if (transfer_list->size == 0)
        wakeup_pid(0);

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
            case MEM_STAT:
                m = "BBBBAAAA";
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
            // transfer_list->size = -1;
            // SV_YIELD_TASK_BLOCK;
        }
        //SV_YIELD_TASK;
    }
    
}

void init_isr(void)
{
    byte_in_id = 4;
    state = RX_READY;
    bufferIndex = 0;
    for (unsigned int i = 0; i < 16; i++)
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
    switch (state)
    {
    case RX_READY:
        unsigned int content = read_data_register();
        in_buffer |= content << ((--byte_in_id) * 8);
        if ((in_buffer & 0xFFFFFF) == magic)
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
            // state = TRANSFER_TASK_BYTES;
            state = RX_READY;
            bufferIndex = 0;
            char s1[9] = {0,0,0,0,0,0,0,0,0};
            unsigned int start_addr = (unsigned int) tInfo->start_adress;
            send_number(start_addr, s1);
            setup_transfer(&s1,9, MEM_ADDRESS);
            // print(s1, 9);

            for (unsigned int i = 0; i < BUFFERSIZE; i++)
                buffer[i] = 0;

            p = tInfo->start_adress;
        }
        break;
    case TRANSFER_TASK_BYTES:
        os_memcpy(p++, read_data_register());
        // setup_transfer(&"foobar", 6);
        // setup_transfer("foobar!\n\r", 9);
        if (++bufferIndex == tInfo->task_size)
        {
            deallocate((unsigned int*) tInfo);
            create_task((void (*)()) tInfo->start_adress, (unsigned int) tInfo->start_adress);
            state = RX_READY;
            bufferIndex = 0;
        }
        break;
    default:
        break;
    }    
}