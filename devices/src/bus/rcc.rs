//!
//! This module provides basic access to the clock bus
//! system. It provides different functions to
//! seperate the activation for the devices actually needed.
//!

//---------------------------------------------------------------//
//----------------------------IMPORTS----------------------------//
//---------------------------------------------------------------//
use super::super::generic::platform::stm32f407ve;
use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::rcc::RCC;
pub mod rcc {

    //---------------------------------------------------------------//
    //-------------------------LOCAL-IMPORTS-------------------------//
    //---------------------------------------------------------------//
    // use super::
    use super::stm32f407ve::adresses;
    use super::stm32f407ve::bitfields;

    use super::primitive_extensions::BitOps;

    use super::RCC;

    ///
    /// Activates the GPIO Clock for the given Port Name .
    ///
    /// # Arguments
    ///
    /// * `port_mnemonic` - A string that describes the port name, e.g. "A" .
    ///
    /// # Returns
    /// * `None`
    ///
    pub unsafe fn activate_gpio_bus_clock(port_mnemonic: &str) {
        let rcc_bus = RCC::new(adresses::RCC);
        match port_mnemonic {
            "A" => (*rcc_bus).ahb1enr.set_bit(0b1 << bitfields::rcc::GPIOPAEN),
            _ => {}
        };
    }

    pub unsafe fn activate_syscfgen_clock() {
        let rcc_bus = RCC::new(adresses::RCC);
        (*rcc_bus).apb2enr.set_bit(0b1 << 14);
    }

    pub unsafe fn reset_syscfgen_clock() {
        let rcc_bus = RCC::new(adresses::RCC);
        (*rcc_bus).apb2rstr.set_bit(0b1 << 14);
    }

    pub unsafe fn activate_usart1_bus_clock() {
        let rcc_bus = RCC::new(adresses::RCC);
        (*rcc_bus).apb2enr.set_bit(0b1 << 4);
    }

    pub unsafe fn activate_dma1_bus_clock() {
        let rcc_bus = RCC::new(adresses::RCC);
        (*rcc_bus).ahb1enr.set_bit(0b1);
    }
}
