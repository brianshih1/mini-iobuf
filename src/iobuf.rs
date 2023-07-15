use intrusive_collections::{intrusive_adapter, LinkedList, LinkedListLink};

use crate::{
    io_allocation_size::IoAllocationSize, io_fragment::IoFragment,
    temporary_buffer::TemporaryBuffer,
};

pub struct IoBuf {
    frags: LinkedList<IoFragmentAdapter>,
    size: usize,
}

intrusive_adapter!(IoFragmentAdapter = Box<IoFragment>: IoFragment { link: LinkedListLink });

impl IoBuf {
    pub fn new() -> Self {
        IoBuf {
            frags: LinkedList::new(IoFragmentAdapter::new()),
            size: 0,
        }
    }

    /// shares the underlying temporary buffers
    pub fn share(&self, pos: usize, len: usize) -> IoBuf {
        let mut ret = IoBuf::new();
        let mut remaining = len;
        let mut pos = pos;
        for fragment in self.frags.iter() {
            if remaining == 0 {
                return ret;
            }
            if pos >= fragment.size() {
                pos -= fragment.size();
                continue;
            }

            let right = std::cmp::min(pos + remaining, fragment.size() - 1);
            let buffer = fragment.share(pos, right);
            ret.append_temporary_buffer(buffer);
            remaining -= right - pos - 1;
        }
        ret
    }

    pub fn available_bytes(&self) -> usize {
        if self.frags.is_empty() {
            return 0;
        }
        let last_fragment = self.frags.back().get().unwrap();
        last_fragment.available_bytes()
    }

    fn get_last_fragment(&self) -> &IoFragment {
        let fragment_cursor = self.frags.back();
        fragment_cursor.get().unwrap()
    }

    /// append src + len into storage
    pub fn append(&mut self, src: *const u8, len: usize) -> () {
        if len <= self.available_bytes() {
            let fragment = self.get_last_fragment();
            self.size += fragment.append(src, len);
            return;
        }
        let mut remaining = len;
        let mut ptr = src;
        while remaining > 0 {
            let allocated_size = self.append_new_fragment(remaining);
            self.get_last_fragment().append(ptr, allocated_size);
            ptr = unsafe { ptr.add(allocated_size) };
            remaining -= allocated_size;
            self.size += allocated_size;
        }
    }

    // append a temporary buffer
    pub fn append_temporary_buffer(&mut self, buffer: TemporaryBuffer) {
        self.append_fragment(IoFragment::from_temporary_buffer(buffer));
    }

    /// append a fragment
    pub fn append_fragment(&mut self, fragment: IoFragment) {
        // TODO: Perform trimming if the fragment isn't empty.
        self.size += fragment.size();
        self.frags.push_back(Box::new(fragment));
    }

    // / creates a new io_fragment and append it to the io_buf
    fn append_new_fragment(&mut self, size: usize) -> usize {
        let next_size = IoAllocationSize::next_allocation_size(size);
        let fragment = IoFragment::new(next_size);
        self.append_fragment(fragment);
        next_size
    }
}
