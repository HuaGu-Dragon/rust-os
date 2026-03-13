#![no_std]
#![no_main]

use core::panic::PanicInfo;

pub mod sync;
pub mod vga_buffer;

#[unsafe(no_mangle)]
#[allow(clippy::empty_loop)]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    println!("The numbers are {} and {}", 42, 1.0 / 3.0);
    print!("Testing print without newline...");
    println!(" Done!");
    panic!("Some panic message");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
