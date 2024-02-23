use base64::prelude::*;

// We will use a null key simply because it is easier to write here, but any key should work
const KEY: [u8; 16] = [0u8; 16];

lazy_static::lazy_static! {
    static ref SECRET: Vec<u8> = BASE64_STANDARD.decode("Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK").expect("base64 secret string is invalid");
}

fn black_box(plaintext: &[u8]) -> Vec<u8> {
    let mut plaintext = plaintext.to_vec();
    plaintext.extend_from_slice(&SECRET);

    ecb::encrypt(&plaintext, &KEY)
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
                break current_size - new_block_start
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
    let plaintext = vec![b'a'; blocksize * 2];
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

fn find_secret_size(blocksize: usize) -> usize {
    let ciphertext = black_box(b"");
    let initial_size = ciphertext.len();
    let mut current_size = 1;

    loop {
        let plaintext = vec![b'a'; current_size];
        let ciphertext = black_box(&plaintext);

        if ciphertext.len() > initial_size {
            break ciphertext.len() - blocksize - current_size
        }

        current_size += 1;
    }
}

fn crack_secret(blocksize: usize, secret_size: usize) -> String {
    let mut known_string = "".to_string();

    for i in 0..secret_size {
        let blocksize = (i / blocksize + 1) * blocksize;

        let plaintext = "A".repeat(blocksize - i - 1).to_string();
        let second_block = black_box(&plaintext.as_bytes())[0..blocksize].to_vec();

        for guess in (0..127u8).map(|x| x as char) {
            let plaintext = plaintext.clone() + &known_string + &guess.to_string();
            println!("{}", plaintext);

            let ciphertext = black_box(plaintext.as_bytes());

            if &ciphertext[0..blocksize] == second_block.as_slice() {
                known_string = known_string + &guess.to_string();
                break;
            }
        };
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
    let secret_size = find_secret_size(blocksize);

    println!("The secret string is {} bytes long", secret_size);
    let secret = crack_secret(blocksize, secret_size);

    println!("The secret string is: {}", secret);
}
