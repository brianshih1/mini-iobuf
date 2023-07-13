use core::slice::SlicePattern;
use std::{
    ptr::NonNull,
    sync::{atomic::AtomicUsize, Arc},
};

struct TemporaryBuffer {
    ptr: NonNull<BufferInternal>,
}

struct BufferInternal {
    ref_count: AtomicUsize,
    data: Vec<u8>,
}

impl TemporaryBuffer {
    /// Creates a temporary buffer of a specified size
    pub fn new(size: usize) -> Self {
        todo!()
    }

    /// Creates a `temporary_buffer` containing a copy of the provided data
    ///
    pub fn new_and_copy_data(data: *mut Vec<u8>, size: usize) -> Self {
        todo!()
    }

    /// Create a new temporary_buffer referring to the same underlying data.
    /// The underlying deleter will not be destroyed until both the original and the clone have
    /// been destroyed.
    pub fn share(&self) -> TemporaryBuffer {
        todo!()
    }

    /// Reads the data in the window as an immutable slice.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { self.ptr.as_ref().data.as_slice() }
    }

    /// Gets a pointer to the beginning of the buffer.
    pub fn begin(&self) -> *const u8 {
        unsafe { self.ptr.as_ref().data.as_ptr() }
    }

    /// Create a new temporary buffer object referring to a substring of the
    /// same underlying data.  The underlying data will not be destroyed
    /// until both the original and the clone have been destroyed.
    pub fn share_slice(&self, pos: usize, len: usize) -> TemporaryBuffer {
        todo!()
    }

    /// Gets a writable pointer to the beginning of the buffer.  Use only
    /// when you are certain no user expects the buffer data not to change.
    pub fn get_write(&self) -> *mut Vec<u8> {
        todo!()
    }
}
