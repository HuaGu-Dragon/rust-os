#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

use rust_os::{exit_qemu, serial_print, serial_println, sync::lazy_lock::LazyLock};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static TEST_IDT: LazyLock<InterruptDescriptorTable> = LazyLock::new(|| {
    let mut idt = InterruptDescriptorTable::new();
    unsafe {
        idt.double_fault
            .set_handler_fn(test_double_fault_handle)
            .set_stack_index(rust_os::gdt::DOUBLE_FAULT_IST_INDEX)
    };

    idt
});

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("test {:.<60}....", "stack_overflow::stack_overflow");

    rust_os::gdt::init();
    init_test_idt();

    // trigger a stack overflow
    stack_overflow();

    panic!("Execution continued after stack overflow");
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed

    unsafe { core::ptr::read_volatile(core::ptr::null::<u8>()) }; // prevent tail recursion optimizations
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

#[allow(clippy::empty_loop)]
extern "x86-interrupt" fn test_double_fault_handle(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(rust_os::QemuExitCode::Success);

    loop {}
}
