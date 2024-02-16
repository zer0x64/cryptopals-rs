use rand::prelude::*;

fn black_box(plaintext: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut key = [0u8; 16];
    rng.fill(&mut key);

    let mut ciphertext = if rng.gen() {
        let mut iv = [0u8; 16];
        rng.fill(&mut iv);

        print!("Used: CBC ");
        cbc::encrypt(plaintext, &key, &iv)
    } else {
        print!("Used: ECB ");
        ecb::encrypt(plaintext, &key)
    };

    // Add a random prefix and suffix
    let prefix_len = rng.gen_range(5..=10);
    let suffix_len = rng.gen_range(5..=10);

    let mut prefix = vec![0u8; prefix_len];
    let mut suffix = vec![0u8; suffix_len];

    rng.fill(prefix.as_mut_slice());
    rng.fill(suffix.as_mut_slice());

    prefix.append(&mut ciphertext);
    prefix.append(&mut suffix);

    prefix
}

fn oracle() {
    let data = [b'A'; 32];

    let ciphertext = black_box(&data);

    for i in 0..ciphertext.len() - 16 {
        if is_sub(&ciphertext[i + 16..], &ciphertext[i..i + 16]) {
            println!("Oracle: ECB");
            return;
        }
    }

    println!("Oracle: CBC");
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
    for _ in 0..100 {
        oracle();
    }
}
