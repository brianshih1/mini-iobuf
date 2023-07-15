/// These constants are borrowed from Redpanda, which is computed from
/// folly::vector's 1.5 growth rule
static ALLOC_TABLE: [usize; 15] = [
    512, 768, 1152, 1728, 2592, 3888, 5832, 8748, 13122, 19683, 29525, 44288, 66432, 99648, 131072,
];

pub struct IoAllocationSize {}

impl IoAllocationSize {
    // finds the next allocation size, potentially smaller than size.
    pub fn next_allocation_size(size: usize) -> usize {
        for alloc_size in ALLOC_TABLE.iter() {
            if *alloc_size >= size {
                return *alloc_size;
            }
        }
        *ALLOC_TABLE.last().unwrap()
    }
}
