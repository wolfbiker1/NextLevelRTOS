extern crate cpu;

//---------------------------------------------------------------//
//-----------------------STRUCT-DEFINITONS-----------------------//
//---------------------------------------------------------------//
#[repr(C)]
pub struct Frame {
    buffer: [u32; 256], /* core::alloc::Layout */
    initialized_core_registers: cpu::core::CoreRegister,
}

//---------------------------------------------------------------//
//---------------------STRUCT-IMPLEMENTATIONS--------------------//
//---------------------------------------------------------------//
impl Frame {
    pub fn new(core: cpu::core::CoreRegister, buffer_size: u32) -> Option<Self> {
        let dynamic_buffer = core::alloc::Layout::from_size_align(buffer_size as usize, 4);
        match dynamic_buffer {
            Ok(buffer) => Some(Frame {
                buffer: unsafe { core::mem::zeroed() },
                initialized_core_registers: core,
            }),
            Err(_) => {
                // todo: print kernel panic over serial later on
                None
            }
        }
    }

    pub fn get_frame_size(&mut self) -> u32 {
        core::mem::size_of::<cpu::core::CoreRegister>() as u32
    }

    pub fn set_target_addr(&mut self, target: u32) {
        self.initialized_core_registers.pc = target;
        // just for testing
        // self.initialized_core_registers.lr = target;
        self.initialized_core_registers.psr = 0x21000000;
    }
    pub fn get_target_addr(&mut self) -> u32 {
        self.initialized_core_registers.pc
    }

    pub fn set_end_destination_addr(&mut self, destination: u32) {
        self.initialized_core_registers.lr = destination;
    }

    pub fn get_r4_location(&self) -> u32 {
        core::ptr::addr_of!(self.initialized_core_registers.r4) as u32
    }
}
