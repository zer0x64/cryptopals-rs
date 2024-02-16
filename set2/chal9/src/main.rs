use anyhow::Result;

fn main() -> Result<()> {
    let mut plaintext = b"YELLOW SUBMARINE".to_vec();

    pkcs7::pad(&mut plaintext, 20);

    assert_eq!(&plaintext, b"YELLOW SUBMARINE\x04\x04\x04\x04");

    pkcs7::unpad(&mut plaintext)?;

    assert_eq!(&plaintext, b"YELLOW SUBMARINE");

    Ok(())
}
