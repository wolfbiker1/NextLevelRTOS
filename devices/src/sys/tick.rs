use crate::generic::traits::primitive_extensions::BitOps;
// use crate::registerblocks::sys;

use super::super::generic::cpu::c_adresses::STK;

// use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::sys::SYSTICK;

pub fn init_systick(period_ms: u32) {
    let cycles_until_zero = period_ms * 8000;
    let systick_block = SYSTICK::new(STK);

    systick_block.load.clear_bit(0x00FF_FFFF);
    systick_block.load.set_bit(cycles_until_zero);

    systick_block.val.clear_bit(0x00FF_FFFF);
    systick_block.ctrl.clear_bit(0b111);
    systick_block.ctrl.set_bit(0b110);
}

pub fn enable_systick() {
    unsafe {
        core::ptr::write_volatile(
            STK as *mut u32,
            core::ptr::read_volatile(STK as *const u32) | 0b1,
        );
    }
}

pub fn disable_systick() {
    unsafe {
        core::ptr::write_volatile(
            STK as *mut u32,
            core::ptr::read_volatile(STK as *const u32) & !0b1,
        );
    }
}
