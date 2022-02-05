//!
//! This file contains a struct containing the registers for the USART device. The fields of the struct are in C presentation
//! to prevent compiler mangling. The fields then match the offsets of the according register.
//!
#[repr(C)]
pub struct USART {
    pub sr: u32,
    pub dr: u32,
    pub brr: u32,
    pub cr1: u32,
    pub cr2: u32,
    pub cr3: u32,
    pub gtpr: u32,
}

impl USART {
    ///
    /// Returns a new USART Struct based on the registers base adress. This adress gets
    /// casted to the struct, as a result the first field will equals the base
    /// adress. The following ones are stacked ontop each other with an offset of
    /// 4 byte / 32 bit.
    ///
    pub fn new(usart_base_adress: u32) -> &'static USART {
        unsafe { &mut *(usart_base_adress as *mut USART) }
    }
}
