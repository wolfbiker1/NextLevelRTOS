#ifndef SCHEDULER_H
#define SCHEDULER_H
#include "data/queue.h"
#include "process/tcb.h"
#include "lang.h"
#include "exception.h"

// extern void __trap(TrapType_t, unsigned int);
typedef struct proc_stats {
    unsigned int num_of_hardfaults;
    unsigned int started_tasks;
    unsigned int finished_tasks;
} ProcessStats_t;

extern ProcessStats_t* process_stats;
extern Queue_t* task_queue;
extern Node_t* currently_running;
extern Queue_t* waiting_tasks;
extern void (*switch_task)();

static inline __attribute__((always_inline)) void wakeup_pid(unsigned int pid)
{
    Node_t *q = currently_running;
    while (1)
    {
        if (((Tcb_t*)q->data)->pid == pid)
        {
            ((Tcb_t*)q->data)->task_state = READY;
            return;
        }
        q = q->next;
    }
}

static inline __attribute__((always_inline)) void move_to_waiting(void)
{
    Node_t* old_element = dequeue_element(task_queue, currently_running);
    enqueue_node(waiting_tasks, old_element);    
}


void next_task(void);
void policy_round_robin(void);
void remove_current_task(void);
void do_selfcheck_scheduler(void);
void init_scheduler(void);
void insert_scheduled_task(Tcb_t*);
void remove_scheduled_task(void);
void run_scheduler(void);
void wakeup_pid(unsigned int);
void block_current_task(void);
void invalidate_current_task(void);
void unblock_task(unsigned int);
void load_task(void);

#endif