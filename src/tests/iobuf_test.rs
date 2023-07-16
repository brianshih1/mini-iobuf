use crate::iobuf::IoBuf;

use super::utils::generate_random_u8_vec;

#[test]
fn test() {
    let mut iobuf = IoBuf::new();
    let values = generate_random_u8_vec(1000);
    let ptr = values.as_ptr();
    let size = values.len();
    iobuf.append(ptr, size);
    {
        iobuf.share(0, 50);
    }
    iobuf.share(0, 1000);
}
