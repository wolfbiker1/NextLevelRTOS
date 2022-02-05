//!
//! This file contains a struct containing the registers for the rcc bus. The fields of the struct are in C presentation
//! to prevent compiler mangling. The fields then match the offsets of the according register.
//!

//---------------------------------------------------------------//
//-----------------------STRUCT-DEFINITONS-----------------------//
//---------------------------------------------------------------//
#[repr(C)]
pub struct RCC {
    pub cr: u32,
    pub pllcfgr: u32,
    pub cfgr: u32,
    pub cir: u32,
    pub ahb1rstr: u32,
    pub ahb2rstr: u32,
    pub ahb3rstr: u32,
    // RO Register
    pub reserved0: u32,
    pub apb1rstr: u32,
    pub apb2rstr: u32,
    pub reserved1: u32,
    pub reserved2: u32,
    pub ahb1enr: u32,
    pub ahb2enr: u32,
    pub ahb3enr: u32,
    pub reserved3: u32,
    pub apb1enr: u32,
    pub apb2enr: u32,
    pub reserved4: u32,
    pub reserved5: u32,
    pub ahb1lpenr: u32,
    pub ahb2lpenr: u32,
    pub ahb3lpenr: u32,
    pub reserved6: u32,
    pub apb1lpenr: u32,
    pub apb2lpenr: u32,
    pub reserved7: u32,
    pub reserved8: u32,
    pub bdcr: u32,
    pub csr: u32,
    pub reserved9: u32,
    pub reserved10: u32,
    pub sscgrg: u32,
    pub plli2scfgr: u32,
}

//---------------------------------------------------------------//
//---------------------STRUCT-IMPLEMENTATIONS--------------------//
//---------------------------------------------------------------//
impl RCC {
    ///
    /// Returns a new RCC Struct based on the registers base adress. This adress gets
    /// casted to the struct, as a result the first field will equals the base
    /// adress. The following ones are stacked ontop each other with an offset of
    /// 4 byte / 32 bit.
    ///
    pub fn new(rcc_base_adress: u32) -> *mut RCC {
        unsafe { &mut *(rcc_base_adress as *mut RCC) }
    }
}
