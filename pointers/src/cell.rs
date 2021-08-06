use std::cell::UnsafeCell;
// Never allowed to cast a shared reference to an exclusive reference

// Modify a value through a shared reference
// Because cannot be shared between threads (!Sync).
//    -> Cannot be changed concurrently
// Never gives out a reference to the value in the store
//    -> Thus can replace it any time (get copies it)
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// implied by UnsafeCell
// impl<T> !Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell { value: UnsafeCell::new(value) }
    }

    pub fn set(&self, value: T) {
        // SAFETY: We know no-one else is concurrently mutating self.value (because !Sync)
        // SAFETY: We know we're not invalidating any references, because we never give any out
        unsafe { *self.value.get() = value };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no-one else is modifying this value, since only this thread can mutate
        // (because !Sync), and it is executing this function instead.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
//    use super::Cell;
//    use std::thread;
//    use std::sync::Arc;
//    
//    // Compiler blocks because !Sync
//    #[test]
//    fn bad() {
//        let x = Arc::new(Cell::new(42));
//        let x1 = Arc::clone(&x);
//        thread::spawn(|| {
//            x.set(43);
//        });
//        let x2 = Arc::clone(&x);
//        thread::spawn(|| {
//            x2.set(44);
//        });
//    }
}
