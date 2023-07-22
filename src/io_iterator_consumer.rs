use std::ops::Add;

use intrusive_collections::linked_list::Iter;

use crate::{io_fragment::IoFragment, iobuf::IoFragmentAdapter};

pub type IoFragmentIter<'a> = Iter<'a, IoFragmentAdapter>;

pub struct IoIteratorConsumer<'a> {
    frag_it: IoFragmentIter<'a>,
    current_frag: Option<&'a IoFragment>,
    frag_index: Option<*const u8>,
}

impl<'a> IoIteratorConsumer<'a> {
    pub fn new(mut begin: IoFragmentIter<'a>) -> Self {
        let fragment = begin.next();
        let frag_index = fragment.map(|frag| frag.get_start());

        IoIteratorConsumer {
            frag_it: begin,
            current_frag: fragment,
            frag_index,
        }
    }

    pub fn consume_to_arr(&mut self, n: usize) -> Vec<u8> {
        let mut ret = Vec::with_capacity(n);
        let offset: *mut u8 = ret.as_mut_ptr();

        self.consume(n, |ptr, size| unsafe {
            std::ptr::copy_nonoverlapping(ptr, offset.add(ret.len()), size);
        });
        unsafe { ret.set_len(n) };
        ret
    }

    /// takes a callback, iterate over the fragments and
    /// consume n bytes of data by invoking the callback chunks
    /// at a time
    pub fn consume<T>(&mut self, n: usize, consumer: T)
    where
        T: Fn(*const u8, usize) -> (),
    {
        let mut consumed = 0;

        while self.current_frag.is_some() && consumed < n {
            let segment_bytes_left = self.segment_bytes_left();

            if segment_bytes_left == 0 {
                self.current_frag = self.frag_it.next();
                self.frag_index = self.current_frag.map(|frag| frag.get_start());
                continue;
            }
            let step = std::cmp::min(segment_bytes_left, n - consumed);
            let frag_index = self.frag_index.unwrap();
            consumer(frag_index, step);
            self.frag_index = Some(unsafe { frag_index.add(step) });
            consumed += step;
        }
    }

    /// represents how many bytes haven't been consumer by the iterator in
    /// the current fragment
    fn segment_bytes_left(&self) -> usize {
        let frag = self.current_frag.unwrap();
        let frag_end_index = unsafe { frag.get_start().add(frag.size()) };
        let frag_index = self.frag_index.unwrap();

        unsafe { frag_end_index.offset_from(frag_index).unsigned_abs() }
    }

    pub fn skip(&self, n: usize) -> () {}
}
