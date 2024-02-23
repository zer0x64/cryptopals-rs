use base64::prelude::*;
use rand::prelude::*;

// We will use a null key simply because it is easier to write here, but any key should work
const KEY: [u8; 16] = [0u8; 16];

lazy_static::lazy_static! {
    static ref SECRET: Vec<u8> = BASE64_STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").expect("base64 secret string is invalid");
    static ref PREFIX: Vec<u8> = {
        let mut rng = thread_rng();

        let prefix_len = rng.gen_range(0..100);

        let mut prefix = vec![0u8; prefix_len];
        rng.fill(prefix.as_mut_slice());

        prefix
    };
}

fn black_box(plaintext: &[u8]) -> Vec<u8> {
    let mut plaintext = plaintext.to_vec();
    plaintext.extend_from_slice(&SECRET);

    let mut prefix = PREFIX.clone();
    prefix.extend(plaintext);

    ecb::encrypt(&prefix, &KEY)
}

fn find_blocksize() -> usize {
    let ciphertext = black_box(b"");

    let mut reset = false;
    let mut new_block_start = 0;
    let mut last_size = ciphertext.len();
    let mut current_size = 1;

    loop {
        let plaintext = vec![b'a'; current_size];
        let ciphertext = black_box(&plaintext);

        if ciphertext.len() > last_size {
            if reset {
                break current_size - new_block_start;
            } else {
                last_size = ciphertext.len();
                new_block_start = current_size;
                reset = true;
            }
        }

        current_size += 1;
    }
}

fn detect_ecb(blocksize: usize) -> bool {
    let plaintext = vec![b'a'; blocksize * 3];
    let ciphertext = black_box(&plaintext);

    for i in 0..ciphertext.len() - blocksize {
        if is_sub(&ciphertext[i + blocksize..], &ciphertext[i..i + blocksize]) {
            return true;
        }
    }

    false
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

fn find_prefix_size(blocksize: usize) -> usize {
    let mut current_size = 1;

    loop {
        let plaintext = vec![b'A'; blocksize * 2 + current_size];
        let ciphertext = black_box(&plaintext);

        for i in 0..(ciphertext.len() - blocksize) / (blocksize + 1) {
            if is_sub(
                &ciphertext[blocksize * (i + 1)..],
                &ciphertext[blocksize * i..blocksize * (i + 1)],
            ) {
                return (i * blocksize) - (current_size % blocksize);
            }
        }

        current_size += 1;
    }
}

fn find_secret_size(blocksize: usize, prefix_size: usize) -> usize {
    let plaintext = vec![b'A'; blocksize - prefix_size % blocksize];

    let ciphertext = black_box(&plaintext);
    let initial_size = ciphertext.len();
    let mut current_size = 1;

    loop {
        let plaintext = vec![b'A'; blocksize - prefix_size % blocksize + current_size];
        let ciphertext = black_box(&plaintext);

        if ciphertext.len() > initial_size {
            break ciphertext.len()
                - blocksize
                - current_size
                - (prefix_size / blocksize + 1) * blocksize;
        }

        current_size += 1;
    }
}

fn crack_secret(blocksize: usize, prefix_size: usize, secret_size: usize) -> String {
    let mut known_string = "".to_string();

    for i in 0..secret_size {
        let blocksize = (i / blocksize + 1) * blocksize;
        let offset = (prefix_size / blocksize + 1) * blocksize;

        let plaintext = "A"
            .repeat(blocksize - prefix_size % blocksize + blocksize - i - 1)
            .to_string();
        let second_block = black_box(&plaintext.as_bytes())[offset..offset + blocksize].to_vec();

        for guess in (0..127u8).map(|x| x as char) {
            let plaintext = plaintext.clone() + &known_string + &guess.to_string();
            println!("{}", plaintext);

            let ciphertext = black_box(plaintext.as_bytes());

            if &ciphertext[offset..offset + blocksize] == second_block.as_slice() {
                known_string = known_string + &guess.to_string();
                break;
            }
        }
    }

    known_string
}

fn main() {
    let blocksize = find_blocksize();
    println!("Blocksize: {}", blocksize);

    if !detect_ecb(blocksize) {
        println!("The black box doesn't use ECB! Aborting...");
        return;
    }

    println!("The black box uses ECB!");

    let prefix_size = find_prefix_size(blocksize);

    println!("The prefix is {} bytes long", prefix_size);

    let secret_size = find_secret_size(blocksize, prefix_size);

    println!("The secret string is {} bytes long", secret_size);
    let secret = crack_secret(blocksize, prefix_size, secret_size);

    println!("The secret string is: {}", secret);
}
