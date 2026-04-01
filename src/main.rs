#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(test)]
use crate::debug::Testable;

mod debug;
pub mod port;
mod serial;
pub mod sync;
pub mod vga_buffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

#[unsafe(no_mangle)]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    let mut port = port::Port::new(0xF4);
    unsafe { port.write(exit_code as u32) };
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("{}", info);
    loop {}
}

#[cfg(not(test))]
fn main() {
    println!("Hello World!");
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    tests.iter().for_each(|t| t.run());

    exit_qemu(QemuExitCode::Success);
}
