#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[cfg(test)]
use crate::debug::Testable;

mod debug;
mod serial;
pub mod sync;
pub mod vga_buffer;

#[unsafe(no_mangle)]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
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
}
