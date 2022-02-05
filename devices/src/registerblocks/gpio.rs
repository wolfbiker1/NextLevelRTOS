//!
//! This file contains a struct containing the registers for the gpio device. The fields of the struct are in C presentation
//! to prevent compiler mangling. The fields then match the offsets of the according register.
//!

//---------------------------------------------------------------//
//-----------------------STRUCT-DEFINITONS-----------------------//
//---------------------------------------------------------------//
#[repr(C)]
pub struct GPIO {
    pub moder: u32,
    pub otyper: u32,
    pub ospeedr: u32,
    pub pupdr: u32,
    pub idr: u32,
    pub odr: u32,
    pub bsrr: u32,
    pub lckr: u32,
    pub afrl: u32,
    pub afrh: u32,
    pub brr: u32,
}

//---------------------------------------------------------------//
//---------------------STRUCT-IMPLEMENTATIONS--------------------//
//---------------------------------------------------------------//
impl GPIO {
    ///
    /// Returns a new GPIO Struct based on the registers base adress. This adress gets
    /// casted to the struct, as a result the first field will equals the base
    /// adress. The following ones are stacked ontop each other with an offset of
    /// 4 byte / 32 bit.
    ///
    pub fn new(gpio_base_adress: u32) -> &'static GPIO {
        unsafe { &mut *(gpio_base_adress as *mut GPIO) }
    }
}
