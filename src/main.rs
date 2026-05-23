#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[unsafe(no_mangle)]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
fn main() {
    use rust_os::println;

    println!("Hello World!");

    rust_os::init();

    #[allow(unconditional_recursion)]
    fn stack_overflow() {
        stack_overflow();
    }

    stack_overflow();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use rust_os::println;

    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}
