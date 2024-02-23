fn main() {
    let test_data = b"test data";

    let mut data = test_data.to_vec();

    pkcs7::pad(&mut data, 16);
    pkcs7::unpad(&mut data).unwrap();

    assert_eq!(test_data.as_slice(), &data);

    assert!(pkcs7::unpad(&mut data).is_err())
}
