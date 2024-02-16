use anyhow::Result;
use base64::prelude::*;

fn get_ciphertext() -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(include_str!("../ciphertext.txt").replace("\n", ""))?)
}

fn main() -> Result<()> {
    let ciphertext = get_ciphertext()?;
    let key = b"YELLOW SUBMARINE";

    let plaintext = ecb::decrypt(&ciphertext, key)?;

    println!("{}", String::from_utf8(plaintext).unwrap());

    Ok(())
}
