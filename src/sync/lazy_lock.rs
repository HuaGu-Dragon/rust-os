use core::{
    cell::UnsafeCell,
    mem::ManuallyDrop,
    ops::Deref,
    sync::atomic::{AtomicU8, Ordering},
};

const INCOMPLETE: u8 = 0;
const RUNNING: u8 = 1;
const COMPLETE: u8 = 2;

pub struct Once {
    state: AtomicU8,
}

impl Default for Once {
    fn default() -> Self {
        Self::new()
    }
}

impl Once {
    pub const fn new() -> Self {
        Self {
            state: AtomicU8::new(INCOMPLETE),
        }
    }

    pub fn call_once<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        if self.state.load(Ordering::Acquire) == COMPLETE {
            return;
        }

        self.call_once_slow(f);
    }

    #[cold]
    fn call_once_slow<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        loop {
            match self.state.compare_exchange(
                INCOMPLETE,
                RUNNING,
                Ordering::Acquire,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    f();
                    self.state.store(COMPLETE, Ordering::Release);
                    return;
                }
                Err(COMPLETE) => {
                    return;
                }
                Err(_) => {
                    while self.state.load(Ordering::Acquire) == RUNNING {
                        core::hint::spin_loop();
                    }
                }
            }
        }
    }

    pub fn is_completed(&self) -> bool {
        self.state.load(Ordering::Acquire) == COMPLETE
    }
}

union Data<T, F> {
    f: ManuallyDrop<F>,
    value: ManuallyDrop<T>,
}

impl<T, F> Data<T, F> {
    const fn new(f: F) -> Self {
        Self {
            f: ManuallyDrop::new(f),
        }
    }
}

pub struct LazyLock<T, F = fn() -> T> {
    once: Once,
    data: UnsafeCell<Data<T, F>>,
}

impl<T, F: FnOnce() -> T> LazyLock<T, F> {
    pub const fn new(f: F) -> Self {
        Self {
            once: Once::new(),
            data: UnsafeCell::new(Data::new(f)),
        }
    }

    pub fn force(this: &Self) -> &T {
        this.once.call_once(|| unsafe {
            let data = &mut *this.data.get();
            let f = ManuallyDrop::take(&mut data.f);

            let value = f();
            data.value = ManuallyDrop::new(value);
        });

        unsafe { &(*this.data.get()).value }
    }

    pub fn is_initialized(&self) -> bool {
        self.once.is_completed()
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyLock<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        Self::force(self)
    }
}

impl<T: Default> Default for LazyLock<T> {
    fn default() -> Self {
        Self::new(T::default)
    }
}

unsafe impl<T, F: Send> Send for LazyLock<T, F> where T: Send {}

unsafe impl<T, F: Send> Sync for LazyLock<T, F> where T: Sync {}

impl<T, F> Drop for LazyLock<T, F> {
    fn drop(&mut self) {
        if self.once.is_completed() {
            unsafe {
                ManuallyDrop::drop(&mut (*self.data.get()).value);
            }
        } else {
            unsafe {
                ManuallyDrop::drop(&mut (*self.data.get()).f);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_lazy_lock_basic() {
        static LAZY: LazyLock<u32> = LazyLock::new(|| 42);
        assert_eq!(*LAZY, 42);
    }

    #[test_case]
    fn test_lazy_lock_with_closure() {
        let lazy: LazyLock<u32, _> = LazyLock::new(|| {
            let x = 10;
            let y = 32;
            x + y
        });
        assert_eq!(*lazy, 42);
    }

    #[test_case]
    fn test_is_initialized() {
        let lazy: LazyLock<u32> = LazyLock::new(|| 42);
        assert!(!lazy.is_initialized());
        let _ = *lazy;
        assert!(lazy.is_initialized());
    }

    #[test_case]
    fn test_force() {
        let lazy: LazyLock<u32> = LazyLock::new(|| 42);
        assert!(!lazy.is_initialized());
        LazyLock::force(&lazy);
        assert!(lazy.is_initialized());
        assert_eq!(*lazy, 42);
    }

    #[test_case]
    fn test_default() {
        let lazy: LazyLock<u32> = LazyLock::default();
        assert_eq!(*lazy, 1);
    }
}
