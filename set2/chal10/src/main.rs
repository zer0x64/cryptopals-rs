use anyhow::Result;
use base64::prelude::*;

fn get_ciphertext() -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(include_str!("../ciphertext.txt").replace("\n", ""))?)
}

fn main() -> Result<()> {
    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
    let ciphertext = get_ciphertext()?;

    let plaintext = cbc::decrypt(&ciphertext, key, &iv)?;

    println!("{}", String::from_utf8(plaintext).unwrap());

    Ok(())
}
