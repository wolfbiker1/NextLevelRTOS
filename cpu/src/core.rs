#[repr(C)]
#[derive(Default)]
pub struct CoreRegister {
    pub r4: u32,
    r5: u32,
    r6: u32,
    r7: u32,
    r8: u32,
    r9: u32,
    r10: u32,
    r11: u32,
    r0: u32,
    r1: u32,
    r2: u32,
    r3: u32,
    r12: u32,
    pub lr: u32,
    pub pc: u32,
    pub psr: u32,
}

impl CoreRegister {
    pub fn new() -> Self {
        Default::default()
    }
}