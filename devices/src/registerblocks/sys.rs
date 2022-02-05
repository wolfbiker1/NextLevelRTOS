//!
//! This file contains a struct containing the registers for the rcc bus. The fields of the struct are in C presentation
//! to prevent compiler mangling. The fields then match the offsets of the according register.
//!

//---------------------------------------------------------------//
//-----------------------STRUCT-DEFINITONS-----------------------//
//---------------------------------------------------------------//
#[repr(C)]
pub struct SYSTICK {
    pub ctrl: u32,
    pub load: u32,
    pub val: u32,
    pub calib: u32,
}

//---------------------------------------------------------------//
//---------------------STRUCT-IMPLEMENTATIONS--------------------//
//---------------------------------------------------------------//
impl SYSTICK {
    pub fn new(stk_base_adress: u32) -> &'static SYSTICK {
        unsafe { &mut *(stk_base_adress as *mut SYSTICK) }
    }
}
