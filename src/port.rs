pub struct Port {
    port: u16,
}

impl Port {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    /// Writes a 32-bit value to the port.
    ///
    /// # Safety
    ///
    /// The caller must ensure that writing `value` to this specific I/O port
    /// is a valid and safe operation. Writing to an incorrect port or writing
    /// an invalid value may cause undefined behavior, hardware faults, or system crashes.
    pub unsafe fn write(&mut self, value: u32) {
        unsafe {
            core::arch::asm!("out dx, eax", in("dx") self.port, in("eax") value, options(nomem, nostack, preserves_flags))
        }
    }
}
