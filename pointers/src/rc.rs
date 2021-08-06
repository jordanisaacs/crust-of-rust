use crate::cell::Cell;
use std::ptr::NonNull;
use std::marker::PhantomData;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

// & is a shared reference, guarantee no exclusive references
// &mut is an exclusive reference (mutable), guaranteed no shared reference
// *const (raw pointer, no guarantees), need an unsafe block to turn into a reference. Only for
// shared references. Can't go from *const to &mut (exclusive reference)
// *mut (raw pointer, no guarantees) is something you *might* be able to mutate. You might have an exclusive reference to.

// unsafe is not necessarily unsafe. it is code that the programmer verifies themselves as safe.

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}
impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });
        // If normal derefence then the box gets dropped when function
        // goes out of scope. Need to not drop the box even though only
        // holding a pointer to it
        Rc {
            // SAFETY: Box does not give us a null pointer
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner))},
            _marker: PhantomData,
        }
    }
}
impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
       let inner = unsafe { self.inner.as_ref() };
       let c = inner.refcount.get();
       inner.refcount.set(c + 1);
       Rc { inner: self.inner, _marker: PhantomData }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // need to drop inner before the Box to make invalid.
            drop(inner);
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc's, and no references to T
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rcs, so don't drop the Box
            inner.refcount.set(c + 1);
        }
    }
}

#[cfg(test)]
mod test {
    // use super::*;


    // Rust does not know inner owns a T, just that there is a pointer to the T. Does not know that
    // when Rc gets dropped it rust doesn't know there is a T that might get dropped.
    // This matters if T might contain lifetimes (not static).
    // Because of Rust drop check. AKA cannot drop a field of a struct before the struct itself is
    // dropped.
    // Without the _marker Rc doesn't contain type T. Therefore can't check and ensure that T was not
    // already dropped.
    // PhantomData tells the compiler to treat the type as though we have one even though we only have
    // a pointer to it.

    // #[test]
    // fn bad() {
    //     let (y,_x);
    //     x = String::from("foo");
    //     y = Rc::new(&x);
    // }
}
