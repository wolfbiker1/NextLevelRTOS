//!
//! This module provides access to the exti controller.
//! The configuration of the interrupt lines can easily be done with
//! a provided builder.
//!

use super::super::bus::rcc;
use super::super::generic::platform::stm32f407ve::adresses;
use super::super::generic::traits::primitive_extensions::BitOps;
use super::super::registerblocks::exti::EXTI;

//---------------------------------------------------------------//
//------------------------TYPE DEFINITONS------------------------//
//---------------------------------------------------------------//
pub enum EdgeDetection {
    RisingEdge,
    FallingEdge,
}

//---------------------------------------------------------------//
//-----------------------STRUCT-DEFINITONS-----------------------//
//---------------------------------------------------------------//
pub struct ExtiConfig {
    controller: &'static EXTI,
    line: u32,
}

//---------------------------------------------------------------//
//---------------------STRUCT-IMPLEMENTATIONS--------------------//
//---------------------------------------------------------------//
impl ExtiConfig {
    ///
    /// Returns a EXTI Object which can be configured to different edge detection modes.
    ///
    /// # Arguments
    ///
    /// * `line_number` - An u32 variable to set the interrupt line.
    ///
    /// # Returns
    /// * `ExtiConfig Struct Object`
    ///
    pub unsafe fn new(line_number: u32) -> ExtiConfig {
        ExtiConfig {
            controller: EXTI::new(adresses::EXTI),
            line: line_number,
        }
    }

    pub fn enable_interrupt(self) -> Self {
        self.controller.imr.set_bit(self.line);
        self
    }

    pub fn disable_interrupt(self) -> Self {
        self.controller.imr.clear_bit(self.line);
        self
    }

    pub fn detect_rising_edge(self) -> Self {
        self.controller.rtsr.set_bit(self.line);
        self
    }

    pub fn detect_falling_edge(self) -> Self {
        self.controller.ftsr.set_bit(self.line);
        self
    }
}
