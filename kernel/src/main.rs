//!
//! Main crate for Kernelimplementation. Depends on runtime
//! and the main function gets called after successful power on.
//!
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(core_intrinsics)]
extern crate devices;
extern crate process;
extern crate runtime;
mod data;
mod mem;
mod proc;
use devices::controller::uart::iostream;
use devices::generic::platform::stm32f407ve::adresses;
use devices::io::gpio::gpio::GpioDevice;
use devices::nvic::exti;

use proc::sched;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn led_off() {
    loop {
        unsafe {
            let mut reg_content = core::ptr::read_volatile(0x4002_0014 as *mut u32);
            reg_content &= !((0b1_u32) << 2);
            core::ptr::write_volatile(0x4002_0014 as *mut u32, reg_content);
        }
    }
}

fn led_on() {
    loop {
        unsafe {
            let mut reg_content = core::ptr::read_volatile(0x4002_0010 as *mut u32);
            reg_content &= !(0xFFFD);
            reg_content >>= 1;
            if reg_content == 0 {
                "is on..".println();
            } else if reg_content == 1 {
                "is off".println();
            }
        }
    }
}

fn calculate_fibonacci() {
    // run 1 time, then destroy
    fibonacci(22);
}

fn user_init() {
    let calculate_fibonacci = process::new_process(
        calculate_fibonacci as *const () as u32,
        sched::destroy as *const () as u32,
    )
    .unwrap();
    let led_off = process::new_process(
        led_off as *const () as u32,
        sched::destroy as *const () as u32,
    )
    .unwrap();
    let led_on = process::new_process(
        led_on as *const () as u32,
        sched::destroy as *const () as u32,
    )
    .unwrap();
    //"spawn process 1".println();
    //sched::spawn(calculate_fibonacci, "calculate_fibonacci");
    //"spawn process 2".println();
    //sched::spawn(led_off, "led_off");
    "spawn process 3".println();
    sched::spawn(led_on, "led_on");
    loop {
        sched::sleep();
    }
}

///
/// Target function after hardware initialization,
/// acts as the first kernel function.
///
#[no_mangle]
pub unsafe fn kernel_init() -> ! {
    mem::malloc::init();
    sched::init();
    devices::sys::tick::init_systick(280);

    let gpio_port_a2 = devices::io::gpio::gpio::GpioDevice::new("A", 2)
        .as_output()
        .as_push_pull();
    gpio_port_a2.turn_on();

    let gpio_port_a3 = devices::io::gpio::gpio::GpioDevice::new("A", 3)
        .as_output()
        .as_push_pull();
    gpio_port_a3.turn_on();

    GpioDevice::new("A", 9)
        .as_alternate_function()
        .as_push_pull()
        .as_af(7);

    GpioDevice::new("A", 1).as_input().as_pull_down();

    let usart = devices::controller::uart::usart::UsartDevice::new(9600);
    usart.enable();

    let early_user_land =
        process::new_process(user_init as *const () as u32, user_init as *const () as u32).unwrap();

    let exti = exti::ExtiConfig::new(1);
    exti.detect_falling_edge().detect_rising_edge().enable_interrupt();
    "hello from trait".println();
    "usart works without errors...".println();
    sched::spawn(early_user_land, "early_user_land");
    sched::start_init_process();

    loop {
        "!! KERNEL PANIC - NO INIT FOUND !!".println();
        asm!("bkpt");
    }
}
