//!
//! This module provides basic access to the clock bus
//! system. It provides different functions to
//! seperate the activation for the devices actually needed.
//!

//---------------------------------------------------------------//
//----------------------------IMPORTS----------------------------//
//---------------------------------------------------------------//
use super::super::generic::platform::stm32f3x;
use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::rcc::RCC;
pub mod rcc {

    //---------------------------------------------------------------//
    //-------------------------LOCAL-IMPORTS-------------------------//
    //---------------------------------------------------------------//
    // use super::
    use super::stm32f3x::bitfields;
    use super::stm32f3x::adresses;

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
            // stm32f4x...
            // "A" => (*rcc_bus).ahb1enr.set_bit(0b1 << bitfields::rcc::GPIOPAEN),
            "A" => (*rcc_bus).ahbenr.set_bit(0b1 << bitfields::rcc::GPIOPAEN),
            _ => {}
        };
    }

    pub unsafe fn activate_timer_bus_clock(tim_port: u32) {
        let rcc_bus = RCC::new(adresses::RCC);
        match tim_port {
            // stm32f4x...
            // "A" => (*rcc_bus).ahb1enr.set_bit(0b1 << bitfields::rcc::GPIOPAEN),
            2 => (*rcc_bus).apb1enr.set_bit(0b1 << bitfields::rcc::TIM2EN),
            _ => {}
        };
    }

    pub unsafe fn activate_usart1_bus_clock() {
        // let rcc_bus = RCC::new(adresses::RCC);
        // (*rcc_bus).apb2enr.set_bit(0b1 << 4); 
    }

    pub unsafe fn activate_dma1_bus_clock() {
        // let rcc_bus = RCC::new(adresses::RCC);
        // (*rcc_bus).ahb1enr.set_bit(0b1);
    }
}
