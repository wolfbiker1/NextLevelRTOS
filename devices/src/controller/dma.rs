use super::super::bus::rcc;
use super::super::generic::platform::stm32f3x;
use super::super::generic::traits::primitive_extensions;
use super::super::registerblocks::dma::DMA;

pub mod dma {
    use super::primitive_extensions::BitOps;
    use super::rcc;
    use super::stm32f3x::adresses;
    use super::DMA;

    pub struct DmaDevice {
        device: DMA,
        // offset_adder: u32,
    }

    impl DmaDevice {
        pub unsafe fn new(channel: u32) -> DmaDevice {
            rcc::rcc::activate_dma1_bus_clock();
            DmaDevice {
                device: DMA::new(adresses::dma::DMA1, channel),
                // offset_adder: 0x14 * (channel - 1)
            }
        }

        pub fn enable(self) -> DmaDevice {
            self.enable_dma_channel();
            self
        }

        pub fn mem_size(self, width: u32) -> DmaDevice {
            self.set_mem_register_width(width);
            self
        }

        pub fn periph_size(self, width: u32) -> DmaDevice {
            self.set_peripherial_register_width(width);
            self
        }

        pub fn disable(self) -> DmaDevice {
            self.disable_dma_channel();
            self
        }

        pub fn peripherial_is_source(self) -> DmaDevice {
            self.read_from_peripherial();
            self
        }

        pub fn memory_is_source(self) -> DmaDevice {
            self.read_from_memory();
            self
        }

        pub fn mem_target_addr(self, mem_adress: u32) -> DmaDevice {
            self.set_memory_adress(mem_adress);
            self
        }

        pub fn peripherial_target_addr(self, peripherial_adress: u32) -> DmaDevice {
            self.set_peripherial_adress(peripherial_adress);
            self
        }

        pub fn transfer_amount(self, amount: u16) -> DmaDevice {
            self.set_number_of_bytes_to_process(amount);
            self
        }

        fn set_mem_register_width(&self, width: u32) {
            let bit_pattern = match width {
                8 => 0b00,
                16 => 0b01,
                32 => 0b10,
                _ => 0b00,
            };
            self.device.dynamic.ccrx.set_bit(bit_pattern << 10);
        }

        fn set_peripherial_register_width(&self, width: u32) {
            let bit_pattern = match width {
                8 => 0b00,
                16 => 0b01,
                32 => 0b10,
                _ => 0b00,
            };
            self.device.dynamic.ccrx.set_bit(bit_pattern << 8);
        }

        fn enable_dma_channel(&self) {
            self.device.dynamic.ccrx.set_bit(0b1);
        }

        fn disable_dma_channel(&self) {
            self.device.dynamic.ccrx.clear_bit(0b1);
        }

        fn read_from_peripherial(&self) {
            self.device.dynamic.ccrx.clear_bit(0b1 << 4);
        }

        fn read_from_memory(&self) {
            self.device.dynamic.ccrx.set_bit(0b1 << 4);
            // when reading from memory, increment ptr also! (MINC)
            self.device.dynamic.ccrx.set_bit(0b1 << 7);
        }

        // addr to read from
        fn set_memory_adress(&self, mem_adress: u32) {
            self.device.dynamic.cmarx.replace_whole_register(mem_adress);
        }

        // destination
        fn set_peripherial_adress(&self, periph_adress: u32) {
            self.device
                .dynamic
                .cparx
                .replace_whole_register(periph_adress);
        }

        fn set_number_of_bytes_to_process(&self, num_of_bytes: u16) {
            self.device
                .dynamic
                .cndtrx
                .write_whole_register(num_of_bytes as u32);
        }
    }
}
