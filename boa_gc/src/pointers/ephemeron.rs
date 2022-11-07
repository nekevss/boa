use crate::{
    finalizer_safe,
    internals::EphemeronBox,
    trace::{Finalize, Trace},
    Allocator, Gc, GcBox, EPHEMERON_QUEUE,
};
use std::cell::Cell;
use std::ptr::NonNull;

pub struct Ephemeron<K: Trace + ?Sized + 'static, V: Trace + ?Sized + 'static> {
    inner_ptr: Cell<NonNull<GcBox<EphemeronBox<K, V>>>>,
}

impl<K: Trace + ?Sized, V: Trace> Ephemeron<K, V> {
    pub fn new(key: &Gc<K>, value: V) -> Self {
        unsafe { value.unroot() };
        Self {
            inner_ptr: Cell::new(Allocator::new(GcBox::new_weak(EphemeronBox::new_pair(
                key, value,
            )))),
        }
    }
}

impl<K: Trace + ?Sized, V: Trace> Ephemeron<K, V> {
    #[inline]
    fn inner_ptr(&self) -> NonNull<GcBox<EphemeronBox<K, V>>> {
        assert!(finalizer_safe());

        self.inner_ptr.get()
    }

    #[inline]
    fn inner(&self) -> &GcBox<EphemeronBox<K, V>> {
        unsafe { &*self.inner_ptr().as_ptr() }
    }

    #[inline]
    pub fn key(&self) -> Option<&K> {
        self.inner().value().key()
    }

    #[inline]
    pub fn value(&self) -> &V {
        self.inner().value().value()
    }
}

impl<K: Trace, V: Trace> Finalize for Ephemeron<K, V> {}

unsafe impl<K: Trace, V: Trace> Trace for Ephemeron<K, V> {
    #[inline]
    unsafe fn trace(&self) {}

    #[inline]
    unsafe fn is_marked_ephemeron(&self) -> bool {
        false
    }

    #[inline]
    unsafe fn weak_trace(&self) {
        EPHEMERON_QUEUE.with(|q| {
            let mut queue = q.take().expect("queue is initialized by weak_trace");
            queue.push(self.inner_ptr())
        })
    }

    #[inline]
    unsafe fn root(&self) {}

    #[inline]
    unsafe fn unroot(&self) {}

    #[inline]
    fn run_finalizer(&self) {
        Finalize::finalize(self)
    }
}
