use std::alloc::{alloc, Layout};
use std::sync::atomic::fence;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::{
    ptr::NonNull,
    sync::{atomic::AtomicUsize, Arc},
};

pub struct TemporaryBuffer {
    ref_count: NonNull<BufferInternal>,
    size: usize,
    buffer: *mut u8,
}

struct BufferInternal {
    ref_counter: AtomicUsize,
}

impl TemporaryBuffer {
    /// Creates a temporary buffer of a specified size, in bytes
    pub fn new(size: usize) -> Self {
        let layout = Layout::array::<u8>(size).unwrap();
        let buffer = unsafe { alloc(layout) };
        TemporaryBuffer {
            ref_count: NonNull::from(Box::leak(Box::new(BufferInternal {
                ref_counter: AtomicUsize::new(1),
            }))),
            size,
            buffer,
        }
    }

    /// Creates a `temporary_buffer` containing a copy of the provided data
    pub fn new_and_copy_data(data: *mut u8, size: usize) -> Self {
        todo!()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn ref_counter(&self) -> &BufferInternal {
        unsafe { self.ref_count.as_ref() }
    }

    pub fn get_ref_count(&self) -> usize {
        self.ref_counter().ref_counter.load(Relaxed)
    }

    /// Create a new temporary_buffer referring to the same underlying data.
    /// The underlying deleter will not be destroyed until both the original and the clone have
    /// been destroyed.
    pub fn share(&self) -> TemporaryBuffer {
        if self.ref_counter().ref_counter.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        TemporaryBuffer {
            ref_count: self.ref_count,
            size: self.size,
            buffer: self.buffer,
        }
    }

    /// Create a new temporary buffer object referring to a substring of the
    /// same underlying data.  The underlying data will not be destroyed
    /// until both the original and the clone have been destroyed.
    pub fn share_slice(&self, pos: usize, len: usize) -> TemporaryBuffer {
        if self.ref_counter().ref_counter.fetch_add(1, Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        // TODO: Validate that pos + len is in bound
        TemporaryBuffer {
            ref_count: NonNull::from(Box::leak(Box::new(BufferInternal {
                ref_counter: AtomicUsize::new(1),
            }))),
            size: len,
            buffer: unsafe { self.buffer.add(pos) },
        }
    }

    /// Reads the data in the window as an immutable slice.
    pub fn as_slice(&self) -> &[u8] {
        // unsafe { self.ptr.as_ref().data.as_slice() }
        todo!()
    }

    /// Gets a pointer to the beginning of the buffer.
    pub fn begin(&self) -> *const u8 {
        self.buffer
    }

    /// Gets a writable pointer to the beginning of the buffer.  Use only
    /// when you are certain no user expects the buffer data not to change.
    pub fn get_write(&self) -> Option<*mut u8> {
        if self.ref_counter().ref_counter.load(Relaxed) == 1 {
            fence(Acquire);
            Some(self.buffer)
        } else {
            None
        }
    }

    /// Checks whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.size() != 0
    }
}

impl Drop for TemporaryBuffer {
    fn drop(&mut self) {
        if self.ref_counter().ref_counter.fetch_sub(1, Release) == 1 {
            fence(Acquire);
            unsafe {
                drop(Box::from_raw(self.ref_count.as_ptr()));
            }
        }
    }
}
