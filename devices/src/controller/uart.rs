use super::super::generic::platform::stm32f3x;
use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::usart::USART;
use super::super::bus::rcc;
pub mod usart {
    use super::rcc;
    use super::primitive_extensions::BitOps;
    use super::stm32f3x::{adresses};
    use super::USART;
    pub struct UsartDevice {
        device: &'static USART,
        baudrate: u32,
    }

    impl UsartDevice {
        pub unsafe fn new(baudrate: u32) -> UsartDevice {
            rcc::rcc::activate_usart1_bus_clock();
            UsartDevice {
                device: USART::new(adresses::USART1_BASEADRESS),
                baudrate: 8_000_000 / baudrate,
            }
        }

        pub fn dma_tx(self) -> UsartDevice {
            self.enable_dma_tx();
            self
        }

        pub fn enable(&self) -> &UsartDevice {
            self.setup_usart_controller();
            self
        }

        fn enable_dma_tx(&self) {
            self.device.cr3.set_bit(0b1 << 7);
        }

        fn setup_usart_controller(&self) {
            self.device.brr.write_whole_register(self.baudrate);
            self.device.cr1.set_bit(0b1100);
            self.device.cr1.set_bit(0b1);
        }

        pub fn print_str(&self, msg: &str) {
            for c in msg.chars() {
                    self.device.tdr.replace_whole_register(c as u32);
                    while !(self.device.isr.read_register() & 0x80 != 0) {}
            }
        }
    }
}
