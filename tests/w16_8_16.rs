use rc5_test::{rc5_w16, Key, Rc5};

fn make_rc5() -> impl Rc5 {
    let key = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
    rc5_w16(Key::try_from(key.as_ref()).unwrap(), 16).unwrap()
}

#[test]
fn encode_a() {
    let pt = vec![0x00, 0x01, 0x02, 0x03];
    let ct = vec![0x23, 0xA8, 0xD7, 0x2E];

    let res = make_rc5().encode(&pt).unwrap();

    assert_eq!(&ct[..], &res[..]);
}

#[test]
fn decode_a() {
    let pt = vec![0x00, 0x01, 0x02, 0x03];
    let ct = vec![0x23, 0xA8, 0xD7, 0x2E];

    let res = make_rc5().decode(&ct).unwrap();

    assert_eq!(&pt[..], &res[..]);
}
