use aes::cipher::typenum::U16;
use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;

pub fn encrypt_decrypt(plaintext: &[u8], key: &[u8; 16], nonce: &[u8; 8]) -> Vec<u8> {
    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = (0..(plaintext.len() as u64) / 16 + 1)
        .map(|i| {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(nonce);
            block[8..].copy_from_slice(&i.to_le_bytes());

            GenericArray::from(block)
        })
        .collect();

    cipher.encrypt_blocks(&mut blocks);

    let keystream: Vec<u8> = blocks.concat();

    // XOR plaintext with keystream
    plaintext
        .iter()
        .zip(keystream.iter())
        .map(|(x, y)| x ^ y)
        .collect()
}

#[test]
fn test_encrypt_decrypt() {
    use base64::prelude::*;

    let key = b"YELLOW SUBMARINE";
    let nonce = [0u8; 8];

    let ciphertext = BASE64_STANDARD
        .decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==")
        .unwrap();

    let plaintext = encrypt_decrypt(&ciphertext, key, &nonce);

    let plaintext = String::from_utf8(plaintext).unwrap();

    assert_eq!(
        "Yo, VIP Let's kick it Ice, Ice, baby Ice, Ice, baby ",
        plaintext
    );
}
