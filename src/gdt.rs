use x86_64::{VirtAddr, structures::tss::TaskStateSegment};

use crate::sync::lazy_lock::LazyLock;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
static TSS: LazyLock<TaskStateSegment> = LazyLock::new(|| {
    let mut tss = TaskStateSegment::new();
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
        const STACK_SIZE: usize = 4096 * 5;
        static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

        VirtAddr::from_ptr(&raw const STACK) + STACK_SIZE as u64
    };
    tss
});
