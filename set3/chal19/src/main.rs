use std::collections::HashMap;

use base64::prelude::*;

// Any key would work, but we'll use a zero key here because it's easy to write
const KEY: [u8; 16] = [0u8; 16];
const NONCE: [u8; 8] = [0u8; 8];

const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz ,.";

lazy_static::lazy_static! {
    // This is a map of the character frequency of ever english character
    static ref ENGLISH_CHARACTER_FREQUENCIES: HashMap<u8, f64> = {
        let mut m = HashMap::new();
        m.insert(b' ', 0.182884);
        m.insert(b'a', 0.084966);
        m.insert(b'b', 0.020720);
        m.insert(b'c', 0.045388);
        m.insert(b'd', 0.033844);
        m.insert(b'e', 0.111607);
        m.insert(b'f', 0.018121);
        m.insert(b'g', 0.024705);
        m.insert(b'h', 0.030034);
        m.insert(b'i', 0.075448);
        m.insert(b'j', 0.001965);
        m.insert(b'k', 0.011016);
        m.insert(b'l', 0.054893);
        m.insert(b'm', 0.030129);
        m.insert(b'n', 0.066544);
        m.insert(b'o', 0.071635);
        m.insert(b'p', 0.031671);
        m.insert(b'q', 0.001962);
        m.insert(b'r', 0.075809);
        m.insert(b's', 0.057351);
        m.insert(b't', 0.069509);
        m.insert(b'u', 0.036308);
        m.insert(b'v', 0.010074);
        m.insert(b'w', 0.012899);
        m.insert(b'x', 0.002902);
        m.insert(b'y', 0.017779);
        m.insert(b'z', 0.002722);
        m
    };
}

fn get_ciphertexts() -> Vec<Vec<u8>> {
    include_str!("plaintexts.txt")
        .lines()
        .map(|p| BASE64_STANDARD.decode(p).unwrap())
        .map(|p| ctr::encrypt_decrypt(&p, &KEY, &NONCE))
        .collect()
}

fn get_keystream(ciphertexts: &[Vec<u8>]) -> Vec<u8> {
    let min_length = ciphertexts.iter().map(|c| c.len()).min().unwrap();

    (0..min_length)
        .map(|i| {
            (0..=255u8)
                .map(|k| {
                    (
                        k,
                        ciphertexts
                            .iter()
                            .map(|c| {
                                // Lowercase the character
                                let mut char = c[i];
                                if char >= 0x41 && char <= 0x5A {
                                    char |= 0x20
                                };

                                ENGLISH_CHARACTER_FREQUENCIES
                                    .get(&(char ^ k))
                                    .unwrap_or(&0f64)
                            })
                            .sum::<f64>(),
                    )
                })
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
                .unwrap()
                .0
        })
        .collect()
}

fn decrypt_ciphertexts(ciphertexts: &[Vec<u8>], keystream: &[u8]) -> Vec<Vec<u8>> {
    ciphertexts
        .iter()
        .map(|c| c.iter().zip(keystream.iter()).map(|(x, y)| x ^ y).collect())
        .collect()
}

fn main() {
    let ciphertexts = get_ciphertexts();
    let ks = get_keystream(&ciphertexts);

    let plaintexts = decrypt_ciphertexts(&ciphertexts, &ks);

    for p in plaintexts {
        println!("{}", String::from_utf8_lossy(&p))
    }
}
