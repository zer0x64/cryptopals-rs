use anyhow::Result;
use base64::prelude::*;

use aes::cipher::typenum::U16;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, KeyInit};
use aes::Aes128;

fn get_ciphertext() -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(include_str!("../ciphertext.txt").replace("\n", ""))?)
}

fn decrypt_aes128_ecb_pkcs7(ciphertext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = ciphertext
        .chunks_exact(16)
        .map(|x| GenericArray::from(<[u8; 16]>::try_from(x).unwrap()))
        .collect();

    cipher.decrypt_blocks(&mut blocks);

    let mut plaintext = blocks.concat();

    // Handle pkcs7 padding
    plaintext.resize(plaintext.len() - plaintext[plaintext.len() - 1] as usize, 0);

    plaintext
}

fn main() -> Result<()> {
    let ciphertext = get_ciphertext()?;
    let key = b"YELLOW SUBMARINE";

    let plaintext = decrypt_aes128_ecb_pkcs7(&ciphertext, key);

    println!("{}", String::from_utf8(plaintext).unwrap());

    Ok(())
}
