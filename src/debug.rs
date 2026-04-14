pub trait Testable {
    fn run(&self);
}

impl<T: Fn()> Testable for T {
    fn run(&self) {
        use crate::{serial_print, serial_println};
        serial_print!("test {:.<60}....", core::any::type_name::<T>());
        (self)();
        serial_println!("[ok]");
    }
}
