#include "memory.h"
#include "lang.h"
#include "test.h"
#include "process/scheduler.h"
#include "uart.h"
#include "uart_common.h"
#include "data/quicksort.h"

static unsigned int* MEM_TABLE_START = 0;
static unsigned int* USEABLE_MEM_START = 0;
const unsigned int NUM_OF_SLOTS = 60;
unsigned int mutex;
char is_first_round = 1;

MemoryStatistic_t mstat;

void swap(char* buffer)
{
    // char* p = (char*) buffer;
    char tmp;

    // swap 1 - 4
    tmp = *buffer;
    *buffer = *(buffer+3);
    *(buffer+3) = tmp;

    // swap 2 - 3
    tmp = *(buffer+1);
    *(buffer+1) = *(buffer+2);
    *(buffer+2) = tmp;
}


void swap_endianess(char* buffer)
{
    // char* p = (char*) buffer;
    char tmp;

    // swap 1 - 4
    tmp = *buffer;
    *buffer = *(buffer+1);
    *(buffer+1) = tmp;

    // swap 2 - 3
    tmp = *(buffer+2);
    *(buffer+2) = *(buffer+3);
    *(buffer+3) = tmp;
}

void memset_byte(void* dest, unsigned int number_of_bytes, char pattern)
{
    char* dest_1 = (char*) dest;
    for (unsigned int i = 0; i < number_of_bytes; i++)
    {
        *dest_1 = pattern;
        dest_1++;
    }
}

void __attribute__((optimize("O0"))) defrag()
{
    lock_mutex((void*) &mutex);

    int highest_used_entry = -1; 
    while (*(MEM_TABLE_START + ++highest_used_entry) != 0xFFFE){}
    
    quicksort((int*) MEM_TABLE_START, 0, --highest_used_entry, &offset_comparator);
    
    for (int base_index = 0; base_index < highest_used_entry; base_index++)
    {
        unsigned int mementry_ptr_1 = *(MEM_TABLE_START + base_index);
        char gap_found = 0;
        // search free element
        if ((mementry_ptr_1 & 1) != 1 && (mementry_ptr_1 != 0xFFFE)) {
            // search next free elements
            int offset_index = base_index + 1;
            while (1)
            {
                unsigned int mementry_ptr_2 = *(MEM_TABLE_START + offset_index);

                // is occupied
                if ((mementry_ptr_2 & 1) == 1 || (offset_index > highest_used_entry))
                    break;
                else
                {
                    gap_found = 1;
                    offset_index++;
                }
            }

            // neighbour not free
            if (!gap_found)
                continue;
            
            // calc new chunk size
            unsigned int new_size = 0;
            for (int k = base_index; k < offset_index; k++)
                new_size += (*(MEM_TABLE_START + k) & 0xFFFE) >> 1;

            // merge
            *(MEM_TABLE_START + base_index) &= ~(0xFFFF);
            *(MEM_TABLE_START + base_index) |= (new_size << 1);

            // relocate #tetrisstyle
            for (int new_location_start = base_index + 1; new_location_start < highest_used_entry + 1; new_location_start++, offset_index++)
                *(MEM_TABLE_START + new_location_start) = *(MEM_TABLE_START + offset_index);
        } 
    }
    release_mutex((void*) &mutex);
}

int __attribute__((optimize("O0"))) offset_comparator(int a, int b)
{
    int offset_a = (a >> 16);
    int offset_b = (b >> 16);
    if (offset_a > offset_b)
        return 1;
    if (offset_a < offset_b)
        return -1;
    return 0;
}

int size_comparator(int a, int b)
{
    unsigned int size_a = (a & 0xFFFE) >> 1;
    unsigned int size_b = (b & 0xFFFE) >> 1;
    if (size_a > size_b)
        return 1;
    if (size_a < size_b)
        return -1;
    return 0;
}


void init_allocator(unsigned int start_os_section, unsigned int* ram_size) {
    while ((start_os_section & 0x3) != 0) {
        start_os_section += 1;
    }
    MEM_TABLE_START = (unsigned int*) start_os_section;
    USEABLE_MEM_START = MEM_TABLE_START + NUM_OF_SLOTS;

    for (unsigned int index = 0; index < NUM_OF_SLOTS; index += 1)
    {
        *(MEM_TABLE_START + index) = 0x0000FFFE;
    }
    mutex = 0;
    memset_byte((void*) &mstat, sizeof(MemoryStatistic_t), 0); 
    mstat.ram_size = (unsigned int) ram_size;
}

unsigned int __attribute__((optimize("O0"))) deallocate(unsigned int* address) {
    // return;
    lock_mutex((void*) &mutex);

    unsigned int address_offset = (unsigned int) address - (unsigned int) USEABLE_MEM_START;
    for (unsigned int index = 0; index < NUM_OF_SLOTS; index++)
    {
        unsigned int alloc_entry = *(MEM_TABLE_START + index);
        if ((alloc_entry >> 16) == address_offset) {
            mstat.num_of_deallocs++;
            *(MEM_TABLE_START + index) = alloc_entry & ~1;
            release_mutex((void*) &mutex);

            return 1;
        }
    }
    release_mutex((void*) &mutex);

    return 0;
}

void __attribute__ ((cold)) update_memory_statistic(void)
{
    unsigned int msp_position;
    __asm volatile ("mrs %0, msp" : "=r"(msp_position));

    mstat.total_byte_alloced = (RAM_START + mstat.ram_size) - msp_position;
    mstat.total_byte_alloced += (unsigned int) MEM_TABLE_START - RAM_START;
    mstat.total_byte_alloced += NUM_OF_SLOTS * sizeof(unsigned int);

    mstat.waiting_tasks = waiting_tasks->size;
    mstat.running_tasks = running_tasks->size;

    // count alloced spaces
    for (unsigned int index = 0; index < NUM_OF_SLOTS; index += 1)
    {
        unsigned int entry = *(MEM_TABLE_START + index);
        if ((entry & 1) == 1)
            mstat.total_byte_alloced += (entry & 0xFFFE) >> 1;
    }    
    // Node_t* q = running_tasks->head;
    mstat.total_byte_used = 0;

    unsigned int *memory = NULL;

    collect_os_statistics((char*)memory);
    deallocate(memory);
    
    #define BUG
    #ifndef BUG
    do
    {
            Tcb_t* t = q->data;
            unsigned int b = t->memory_lower_bound;
            unsigned int f = t->general.task_info.stack_size;
            unsigned int c = t->sp;
            unsigned int l = b + f - c;
            q = q->next;
            mstat.total_byte_used += l;
    } while (q != running_tasks->head);

    q = waiting_tasks->head;
    do
    {
            Tcb_t* t = q->data;
            mstat.total_byte_used += (t->memory_lower_bound + t->general.task_info.stack_size) - t->sp;
            q = q->next;
    } while (q != waiting_tasks->head);
    #endif
    #undef BUG
}

unsigned int* __attribute__((optimize("O0"))) allocate(unsigned int size) {
    lock_mutex((void*) &mutex);
    unsigned int next_useable_chunk = 0;

    while ((size % 4) != 0) { size++; }

    /*
     *  CHUNK LIST LAYOUT
     *
     *  start_of_memory_block: 0x2000_0100 + OFFSET
     *  | OFFSET , SIZE, IS_OCUPIED | [31..16, 15..1, 0]
     *
     *
     **/
    for (unsigned int index = 0; index < NUM_OF_SLOTS; index++)
    {
        // 47 possible allocs @todo WRONG COUNT!!
        // MemoryEntry_t* memory_entry = (MemoryEntry_t*) ((unsigned int*) *(MEM_TABLE_START + index));
        MemoryEntry_t* memory_entry = (MemoryEntry_t*) (MEM_TABLE_START + index);

        // check if occupied
        if (memory_entry->mementry_fields.is_occupied) {
            // get size and add to offset
            next_useable_chunk += memory_entry->mementry_fields.chunk_size;
            continue;
        }

        // check if chunk was already in use: size is not default 
        if ((memory_entry->mementry_fields.chunk_size != 0x7FFF) && (memory_entry->mementry_fields.chunk_size >= size))
        {
            unsigned int old_size = memory_entry->mementry_fields.chunk_size;

            memory_entry->mementry_fields.chunk_size = 0;
            memory_entry->mementry_fields.is_occupied = 0;
            unsigned int remaining = old_size - size;

            // untested!
            if (remaining > 0)
            {
                unsigned int j = index;

                // @todo: leads to BUSFAULT because may be an infinite loop!
                while (*(MEM_TABLE_START + j) != 0x0000FFFE) { j++; };
                *(MEM_TABLE_START + j) = (((memory_entry->raw >> 16) + size) << 16) | remaining << 1 | 0; 
                // mstat.num_of_fractial_allocs++;
            }

            *(MEM_TABLE_START + index) &= 0xFFFF0000;
            *(MEM_TABLE_START + index) |= (size << 1) | 0x1;
            mstat.num_of_allocs++;
            release_mutex((void*) &mutex);
            return (unsigned int*) ((memory_entry->raw >> 16) + (unsigned int) USEABLE_MEM_START);
        } 
        // check if size fits on new chunk
        if (memory_entry->mementry_fields.chunk_size >= size)
        {
            // update offsetadress, size, mark as occupied
            memory_entry->mementry_fields.base_offset = next_useable_chunk;
            memory_entry->mementry_fields.chunk_size = size;
            memory_entry->mementry_fields.is_occupied = 1;
            // memory_entry->mementry_fields.is_dirty = 1;

            // write back changes
            *(MEM_TABLE_START + index) = memory_entry->raw;
            mstat.num_of_allocs++;
            release_mutex((void*) &mutex);

            return (unsigned int*) (memory_entry->mementry_fields.base_offset + (unsigned int) USEABLE_MEM_START);
        }
        next_useable_chunk += memory_entry->mementry_fields.chunk_size;
    }
    if (is_first_round)
    {
        defrag();
        is_first_round = 0;
        release_mutex((void*) &mutex);
        return allocate(size);
    }
    else
    {
        is_first_round = 1;
        release_mutex((void*) &mutex);
        return NULL;
    }
}