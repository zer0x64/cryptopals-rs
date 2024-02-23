use aes::cipher::typenum::U16;
use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;

pub fn decrypt(ciphertext: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, pkcs7::InvalidPadding> {
    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = ciphertext
        .chunks_exact(16)
        .map(|x| GenericArray::from(<[u8; 16]>::try_from(x).unwrap()))
        .collect();

    cipher.decrypt_blocks(&mut blocks);

    let mut plaintext = blocks.concat();

    // Handle pkcs7 padding
    pkcs7::unpad(&mut plaintext)?;

    Ok(plaintext)
}

pub fn encrypt(plaintext: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut plaintext = plaintext.to_vec();

    // Handle pkcs7 padding
    pkcs7::pad(&mut plaintext, 16);

    let key = GenericArray::from(*key);
    let cipher = Aes128::new(&key);

    let mut blocks: Vec<GenericArray<u8, U16>> = plaintext
        .chunks_exact(16)
        .map(|x| GenericArray::from(<[u8; 16]>::try_from(x).unwrap()))
        .collect();

    cipher.encrypt_blocks(&mut blocks);

    blocks.concat()
}

#[test]
fn test_encrypt_decrypt() {
    let key = b"YELLOW SUBMARINE";

    let plaintext = b"test".to_vec();
    let ciphertext = encrypt(&plaintext, key);

    assert_eq!(&plaintext, &decrypt(&ciphertext, key).unwrap());
}
