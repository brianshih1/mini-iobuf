use crate::temporary_buffer::TemporaryBuffer;

#[test]
fn test_share_ref_count() {
    let buffer = TemporaryBuffer::new(12);
    {
        let second = buffer.share();
        assert_eq!(second.get_ref_count(), 2);
    }
    assert_eq!(buffer.get_ref_count(), 1);
}
