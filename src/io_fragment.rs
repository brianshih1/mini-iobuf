use std::cell::RefCell;

use crate::temporary_buffer::TemporaryBuffer;
use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};

pub struct IoFragment {
    used_bytes: RefCell<usize>,
    buf: TemporaryBuffer,
    pub link: LinkedListLink,
}

impl IoFragment {
    pub fn new(size: usize) -> Self {
        IoFragment {
            used_bytes: RefCell::new(0),
            buf: TemporaryBuffer::new(size),
            link: LinkedListLink::new(),
        }
    }

    /// Initialize fragment from the provided temporary buffer.
    pub fn from_temporary_buffer(buffer: TemporaryBuffer) -> Self {
        IoFragment {
            used_bytes: RefCell::new(0),
            buf: buffer,
            link: LinkedListLink::new(),
        }
    }

    /// Gets a pointer to the beginning of the buffer.
    pub fn get_start(&self) -> *const u8 {
        self.buf.begin()
    }

    fn get_current(&self) -> Option<*mut u8> {
        self.buf.get_write()
    }

    pub fn used_bytes(&self) -> usize {
        *self.used_bytes.borrow()
    }

    fn get_write(&self) -> *mut u8 {
        unsafe { self.get_current().unwrap().add(self.used_bytes()) }
    }

    pub fn is_empty(&self) -> bool {
        self.used_bytes() == 0
    }

    pub fn available_bytes(&self) -> usize {
        self.buf.size() - self.used_bytes()
    }

    // how much data is taken
    pub fn size(&self) -> usize {
        self.used_bytes()
    }

    // how much the temporary buffer can hold
    pub fn capacity(&self) -> usize {
        self.buf.size()
    }

    // Returns how many bytes was appended to the fragment.
    pub fn append(&self, ptr: *const u8, size: usize) -> usize {
        let appended = std::cmp::min(self.available_bytes(), size);
        unsafe {
            std::ptr::copy_nonoverlapping(ptr, self.get_current().unwrap(), size);
        }
        *self.used_bytes.borrow_mut() += appended;
        appended
    }

    pub fn share(&self, pos: usize, len: usize) -> TemporaryBuffer {
        self.buf.share_slice(pos, len)
    }

    pub fn share_whole(&self) -> TemporaryBuffer {
        self.share(0, self.used_bytes())
    }
}
