use std::time::{SystemTime, UNIX_EPOCH};

use mt19937::Mt19937Rng;
use rand::prelude::*;

lazy_static::lazy_static! {
    static ref KEY: u16 = thread_rng().gen();
}

fn encrypt_decrypt(data: &[u8], key: u16) -> Vec<u8> {
    let mut rng = Mt19937Rng::seed_from_u64(key as u64);

    let mut output = vec![0u8; data.len()];
    rng.fill_bytes(&mut output);

    for i in 0..data.len() {
        output[i] ^= data[i];
    }

    output
}

fn part1(data: &[u8]) -> Vec<u8> {
    let prefix_len = thread_rng().gen_range(4..100);
    
    let mut prefix = vec![0u8; prefix_len];
    thread_rng().fill(prefix.as_mut_slice());

    prefix.extend_from_slice(data);

    encrypt_decrypt(&prefix, *KEY)
}

fn part1_solve() -> u16 {
    let data = [0x41u8; 14];
    let ciphertext = part1(&data);

    for i in 0u16..=u16::MAX {
        let test_plaintext = encrypt_decrypt(&ciphertext, i);
        if is_sub(&test_plaintext, &data) {
            return i;
        }
    };

    0
}

fn part2() -> [u8; 32] {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs();

    let mut rng = Mt19937Rng::seed_from_u64(timestamp);

    rng.gen()
}

fn part2_solve() -> bool {
    let token = part2();

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs();

    let mut rng = Mt19937Rng::seed_from_u64(timestamp);
    let output: [u8; 32] = rng.gen();

    is_sub(output.as_slice(), token.as_slice())
}

fn is_sub<T: PartialEq>(mut haystack: &[T], needle: &[T]) -> bool {
    if needle.len() == 0 {
        return true;
    }
    while !haystack.is_empty() {
        if haystack.starts_with(needle) {
            return true;
        }
        haystack = &haystack[1..];
    }
    false
}

fn main() {
    println!("Part 1!");
    println!("Actual key: {}", *KEY);

    println!("Cracked key: {}", part1_solve());

    println!("Is the token valid? {}", part2_solve());
}
