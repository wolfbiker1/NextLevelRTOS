//!
//! This module extends some primitive datatypes with different
//! features. It may be operating system- or low level features
//! like bit operations for u32.
//!
pub mod primitive_extensions {
    pub trait BitOps {
        fn get_addr(&self) -> *const u32;
        fn set_bit(&self, bit_number: u32);
        fn clear_bit(&self, bit_number: u32);
    }

    impl BitOps for u32 {
        fn get_addr(&self) -> *const u32 {
            unsafe { *core::ptr::addr_of!(self) }
        }
        /// Sets a single bit to '1'
        ///
        /// # Arguments
        ///
        /// * `bit_number` - A u32 which represents the bit position to be set (LSB)
        ///
        /// # Returns
        /// * `None`
        ///
        ///
        fn set_bit(&self, bit_number: u32) {
            unsafe {
                let address = self.get_addr();
                core::ptr::write(address as *mut u32, core::ptr::read(address) | bit_number);
            }
        }
        /// Clears out a single bit
        ///
        /// # Arguments
        ///
        /// * `bit_number` - A u32 which represents the bit position to be cleared (LSB)
        ///
        /// # Returns
        /// * `None`
        ///
        ///
        fn clear_bit(&self, bit_number: u32) {
            unsafe {
                let address = self.get_addr();
                core::ptr::write(address as *mut u32, core::ptr::read(address) & !bit_number);
            }
        }
    }
}