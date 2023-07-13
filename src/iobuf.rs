struct IoBuf {}

impl IoBuf {
    /// shares the underlying temporary buffers
    pub fn share(pos: usize, len: usize) -> IoBuf {
        todo!()
    }

    /// append src + len into storage
    pub fn append(src: *const Vec<u8>, len: usize) -> () {
        todo!()
    }
}
