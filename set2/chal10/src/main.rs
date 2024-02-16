use anyhow::Result;
use base64::prelude::*;

use aes::cipher::typenum::U16;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

fn get_ciphertext() -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(include_str!("../ciphertext.txt").replace("\n", ""))?)
}

fn encrypt_aes128_cbc_pkcs7(plaintext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Vec<u8> {
    // Handle pkcs7 padding
    let mut plaintext = plaintext.to_vec();
    pkcs7::pad(&mut plaintext, 16);

    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = plaintext
        .chunks_exact(16)
        .map(|x| GenericArray::from(<[u8; 16]>::try_from(x).unwrap()))
        .collect();

    // XOR with previous ciphertexts
    xor_block_in_place(&mut blocks[0], iv);
    cipher.encrypt_block(&mut blocks[0]);

    for i in 1..blocks.len() {
        let b: [u8; 16] = blocks[i - 1].try_into().unwrap();
        xor_block_in_place(&mut blocks[i], &b);
        cipher.encrypt_block(&mut blocks[i]);
    }

    blocks.concat()
}

fn decrypt_aes128_cbc_pkcs7(ciphertext: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>> {
    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = ciphertext
        .chunks_exact(16)
        .map(|x| GenericArray::from(<[u8; 16]>::try_from(x).unwrap()))
        .collect();

    let ciphertext_blocks = blocks.clone();

    // Decrypt all blocks in parallel
    cipher.decrypt_blocks(&mut blocks);

    // XOR with previous ciphertexts
    xor_block_in_place(&mut blocks[0], iv);
    for i in 0..blocks.len() - 1 {
        let b: [u8; 16] = ciphertext_blocks[i].try_into().unwrap();
        xor_block_in_place(&mut blocks[i + 1], &b);
    }

    let mut plaintext = blocks.concat();

    // Handle pkcs7 padding
    pkcs7::unpad(&mut plaintext)?;

    Ok(plaintext)
}

fn xor_block_in_place(a: &mut GenericArray<u8, U16>, b: &[u8; 16]) {
    for i in 0..16 {
        a[i] ^= b[i];
    }
}

fn main() -> Result<()> {
    let key = b"YELLOW SUBMARINE";
    let iv = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

    let plaintext = b"test".to_vec();
    let ciphertext = encrypt_aes128_cbc_pkcs7(&plaintext, key, iv);

    assert_eq!(&plaintext, &decrypt_aes128_cbc_pkcs7(&ciphertext, key, iv)?);

    let ciphertext = get_ciphertext()?;

    let plaintext = decrypt_aes128_cbc_pkcs7(&ciphertext, key, &iv)?;

    println!("{}", String::from_utf8(plaintext).unwrap());

    Ok(())
}
