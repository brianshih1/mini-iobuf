use crate::temporary_buffer::TemporaryBuffer;

#[test]
fn test_share_ref_count() {
    let buffer = TemporaryBuffer::new(12);
    {
        let second = buffer.share();
        assert_eq!(buffer.get_ref_count(), 2);

        {
            let slice = buffer.share_slice(0, 3);
            assert_eq!(buffer.get_ref_count(), 3);
        }
    }
    assert_eq!(buffer.get_ref_count(), 1);
}

#[test]
fn write() {
    let buffer = TemporaryBuffer::new(12);
    let ptr = buffer.get_write().unwrap();
    let data: Vec<u8> = vec![1, 2, 3];
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr, 3);
    }
}
