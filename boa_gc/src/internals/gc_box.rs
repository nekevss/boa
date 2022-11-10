use crate::Trace;
use std::cell::Cell;
use std::ptr::{self, NonNull};

// Age and Weak Flags
const MARK_MASK: usize = 1 << (usize::BITS - 2);
const WEAK_MASK: usize = 1 << (usize::BITS - 1);
const ROOTS_MASK: usize = !(MARK_MASK | WEAK_MASK);
const ROOTS_MAX: usize = ROOTS_MASK;

pub(crate) struct GcBoxHeader {
    roots: Cell<usize>,
    pub(crate) next: Cell<Option<NonNull<GcBox<dyn Trace>>>>,
}

impl GcBoxHeader {
    #[inline]
    pub(crate) fn new() -> Self {
        GcBoxHeader {
            roots: Cell::new(1),
            next: Cell::new(None),
        }
    }

    #[inline]
    pub(crate) fn new_weak() -> Self {
        // Set weak_flag
        GcBoxHeader {
            roots: Cell::new(WEAK_MASK | 1),
            next: Cell::new(None),
        }
    }

    #[inline]
    pub(crate) fn roots(&self) -> usize {
        self.roots.get() & ROOTS_MASK
    }

    #[inline]
    pub(crate) fn inc_roots(&self) {
        let roots = self.roots.get();

        if (roots & ROOTS_MASK) < ROOTS_MAX {
            self.roots.set(roots + 1);
        } else {
            // TODO: implement a better way to handle root overload
            panic!("roots counter overflow");
        }
    }

    #[inline]
    pub(crate) fn dec_roots(&self) {
        // Underflow check as a stop gap for current issue when dropping
        if self.roots.get() > 0 {
            self.roots.set(self.roots.get() - 1);
        }
    }

    #[inline]
    pub(crate) fn is_marked(&self) -> bool {
        self.roots.get() & MARK_MASK != 0
    }

    #[inline]
    pub(crate) fn mark(&self) {
        self.roots.set(self.roots.get() | MARK_MASK);
    }

    #[inline]
    pub(crate) fn unmark(&self) {
        self.roots.set(self.roots.get() & !MARK_MASK);
    }

    #[inline]
    pub(crate) fn is_ephemeron(&self) -> bool {
        self.roots.get() & WEAK_MASK != 0
    }
}

// NOTE: [repr(C)] is most likely unneeded here, but will keep it for now
/// A garbage collected allocation.
#[repr(C)]
pub(crate) struct GcBox<T: Trace + ?Sized + 'static> {
    pub(crate) header: GcBoxHeader,
    pub(crate) value: T,
}

impl<T: Trace> GcBox<T> {
    pub(crate) fn new(value: T) -> Self {
        GcBox {
            header: GcBoxHeader::new(),
            value,
        }
    }

    pub(crate) fn new_weak(value: T) -> Self {
        GcBox {
            header: GcBoxHeader::new_weak(),
            value,
        }
    }
}

impl<T: Trace + ?Sized> GcBox<T> {
    /// Returns `true` if the two references refer to the same `GcBox`.
    pub(crate) fn ptr_eq(this: &GcBox<T>, other: &GcBox<T>) -> bool {
        // Use .header to ignore fat pointer vtables, to work around
        // https://github.com/rust-lang/rust/issues/46139
        ptr::eq(&this.header, &other.header)
    }

    /// Marks this `GcBox` and marks through its data.
    pub(crate) unsafe fn trace_inner(&self) {
        if !self.header.is_marked() && !self.header.is_ephemeron() {
            self.header.mark();
            // SAFETY: if `GcBox::trace_inner()` has been called, then,
            // this box must have been deemed as reachable via tracing
            // from a root, which by extension means that value has not
            // been dropped either.
            unsafe {
                self.value.trace();
            }
        }
    }

    /// Trace inner data
    pub(crate) fn weak_trace_inner(&self) {
        // SAFETY: if a `GcBox` has `weak_trace_inner` called, then the inner.
        // value must have been deemed as reachable.
        unsafe {
            self.value.weak_trace();
        }
    }

    /// Increases the root count on this `GcBox`.
    /// Roots prevent the `GcBox` from being destroyed by the garbage collector.
    pub(crate) fn root_inner(&self) {
        self.header.inc_roots();
    }

    /// Decreases the root count on this `GcBox`.
    /// Roots prevent the `GcBox` from being destroyed by the garbage collector.
    pub(crate) fn unroot_inner(&self) {
        if !self.header.is_ephemeron() {
            self.header.dec_roots();
        }
    }

    /// Returns a reference to the `GcBox`'s value.
    pub(crate) fn value(&self) -> &T {
        &self.value
    }

    pub(crate) fn is_marked(&self) -> bool {
        self.header.is_marked()
    }
}
