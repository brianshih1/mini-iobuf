use crate::{io_iterator_consumer::IoIteratorConsumer, iobuf::IoBuf};

use super::utils::generate_random_u8_vec;

#[test]
fn test_consume_to_arr() {
    let mut iobuf = IoBuf::new();
    let values1 = generate_random_u8_vec(1000);
    iobuf.append(values1.as_ptr(), values1.len());

    let values2 = generate_random_u8_vec(3000);
    iobuf.append(values2.as_ptr(), values2.len());

    let mut consumer = IoIteratorConsumer::new(iobuf.begin());
    let arr = consumer.consume_to_arr(1000);
    assert_eq!(values1, arr);

    let arr2 = consumer.consume_to_arr(3000);
    assert_eq!(values2, arr2);
}
