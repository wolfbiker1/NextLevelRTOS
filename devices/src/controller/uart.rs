use super::super::bus::rcc;
use super::super::generic::platform::stm32f407ve;
use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::usart::USART;

const USART1_DR: u32 = 0x4001_1004;
const USART1_SR: u32 = 0x4001_1000;

const CRLF: &str = "\r\n";

pub trait iostream {
    fn print(&self);
    fn println(&self);
}

///
/// Appends print methods to &str primitive. The println method
/// sends a newline character and a carriage return after the
/// payload has been succesful transmitted.
///
impl iostream for &str {
    fn print(&self) {
        for c in self.chars() {
            transmit(c as u32);
        }
    }
    fn println(&self) {
        for c in self.chars() {
            transmit(c as u32);
        }
        for c in CRLF.chars() {
            transmit(c as u32);
        }
    }
}

pub mod usart {
    use super::primitive_extensions::BitOps;
    use super::rcc;
    use super::stm32f407ve::adresses;
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
                baudrate,
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
            // bus is feed from HSI CLK with 16 MHZ.
            // how to set brr register:
            // page 981 table 136 entry 2:
            // baud: 104.1875 -> 104 | 16 * 0.1875 : 0d1667
            self.device.brr.set_bit(1667);
            self.device.cr1.set_bit(0b1100);
            self.device.cr1.set_bit(0b1 << 13);
        }
    }
}

///
/// Writes one byte into transmit register und polls interrupt flag until transmission in complete. Acts as
/// as "blocking send".
///
/// # Arguments
///
/// * `c` - A bytee representing the char to be sent.
///
/// # Returns
/// * `Nothing`
///
fn transmit(c: u32) {
    unsafe {
        // currently not clear if buffer needs to be flushed first
        //core::ptr::write_volatile(USART1_DR as *mut u32, core::ptr::read_volatile(USART1_DR as *const u32) & !(0x1FF));
        core::ptr::write_volatile(
            USART1_DR as *mut u32,
            core::ptr::read_volatile(USART1_DR as *const u32) | c,
        );
        while !((core::ptr::read_volatile(USART1_SR as *const u32) & 0x80) != 0) {}
    }
}
