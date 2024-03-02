use base64::prelude::*;

// Any key would work, but we'll use a zero key here because it's easy to write
const KEY: [u8; 16] = [0u8; 16];
const NONCE: [u8; 8] = [0u8; 8];

fn get_ciphertexts() -> Vec<Vec<u8>> {
    include_str!("plaintexts.txt")
        .lines()
        .map(|p| BASE64_STANDARD.decode(p).unwrap())
        .map(|p| ctr::encrypt_decrypt(&p, &KEY, &NONCE))
        .collect()
}

fn find_keystream(ciphertexts: &[Vec<u8>]) -> Vec<u8> {
    let min_length = ciphertexts.iter().map(|c| c.len()).min().unwrap();

    let mut matrix = vec![vec![0u8; ciphertexts.len()]; min_length];
    let mut key = vec![0u8; min_length];

    // Transpose the blocks
    for (i, c) in ciphertexts.iter().enumerate() {
        for (j, val) in c[0..min_length].into_iter().enumerate() {
            matrix[j][i] = *val;
        }
    }

    for (i, c) in matrix.into_iter().enumerate() {
        key[i] = single_byte_xor::brute_force(&c).unwrap().key;
    }

    key
}

fn decrypt_ciphertexts(ciphertexts: &[Vec<u8>], keystream: &[u8]) -> Vec<Vec<u8>> {
    ciphertexts
        .iter()
        .map(|c| c.iter().zip(keystream.iter()).map(|(x, y)| x ^ y).collect())
        .collect()
}

fn main() {
    let ciphertexts = get_ciphertexts();

    let ks = find_keystream(&ciphertexts);

    let plaintexts = decrypt_ciphertexts(&ciphertexts, &ks);

    for p in plaintexts {
        println!("{}", String::from_utf8_lossy(&p))
    }
}
