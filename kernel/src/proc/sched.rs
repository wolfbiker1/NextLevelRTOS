//---------------------------------------------------------------//
//----------------------------IMPORTS----------------------------//
//---------------------------------------------------------------//
use super::super::data::list::List;
use super::task;
use process::blueprint::Frame;

static mut RUNNABLE_TASKS: u32 = 0;
static mut SLEEPING_TASKS: u32 = 0;

extern "C" {
    ///
    /// Points to extern asm instructions.
    ///
    fn __br_to_init();
    fn __trap(f: u32);
    fn __load_process_context(stack_addr: u32);
    fn __save_process_context() -> u32;
}

pub fn init() {
    unsafe { RUNNABLE_TASKS = List::new() };
    unsafe { SLEEPING_TASKS = List::new() };
}

pub fn destroy() {
    unsafe {
        core::intrinsics::volatile_store(
            0xE000_E010 as *mut u32,
            core::ptr::read_volatile(0xE000_E010 as *const u32) & !0b1,
        );
        let list = &mut *(RUNNABLE_TASKS as *mut List);

        // lost forever right now
        list.delete_head_node();

        if list.get_size() == 0 {
            // wake up idle task
            // or: set cpu to sleep
        }

        let p = list.cursor_sp();

        core::intrinsics::volatile_store(
            0xE000_E010 as *mut u32,
            core::ptr::read_volatile(0xE000_E010 as *const u32) | 0b1,
        );
        __trap(p);
    }
}

// experimental
pub fn sleep() {
    unsafe {
        let list_active = &mut *(RUNNABLE_TASKS as *mut List);
        let list_sleeping = &mut *(SLEEPING_TASKS as *mut List);
        //if list.size > 1 {
        //  // todo ...
        //}
    }
}

pub fn shift_task() -> u32 {
    let list = unsafe { &mut *(RUNNABLE_TASKS as *mut List) };
    list.get_next_sp()
}

pub fn start_init_process() {
    unsafe {
        let stackpointer_value = shift_task();

        // start systick timer
        core::ptr::write_volatile(
            0xE000_E010 as *mut u32,
            core::ptr::read_volatile(0xE000_E010 as *const u32) | 0b1,
        );
        __trap(stackpointer_value);
    }
}

pub fn spawn(p: Frame, name: &str) {
    // unwrap task list which contains tcb's of runnable tasks
    let list = unsafe { &mut *(RUNNABLE_TASKS as *mut List) };
    let r4 = p.get_r4_location();
    let pid = list.get_size();
    let tcb = task::create(r4, pid, name);
    list.insert(tcb);
}

pub fn context_switch() {
    unsafe {
        let old_sp = __save_process_context();
        let list = &mut *(RUNNABLE_TASKS as *mut List);
        list.update_tcb(old_sp);
        let sp = list.get_next_sp();

        __load_process_context(sp);
    }
}

#[no_mangle]
pub extern "C" fn SysTick() {
    context_switch();
}

#[no_mangle]
pub extern "C" fn SVCall() {
    unsafe { __br_to_init() };
}
