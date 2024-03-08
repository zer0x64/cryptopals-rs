use base64::prelude::*;
use rand::prelude::*;

const RANDOM_STRINGS: [&'static str; 10] = [
    "MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=",
    "MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=",
    "MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==",
    "MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==",
    "MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl",
    "MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==",
    "MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==",
    "MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=",
    "MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=",
    "MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93",
];

lazy_static::lazy_static! {
    static ref KEY: [u8; 16] = {
        let mut key = [0u8; 16];
        thread_rng().fill(&mut key);

        key
    };
}

fn get_secret() -> Vec<u8> {
    let secret = RANDOM_STRINGS.choose(&mut thread_rng()).unwrap();
    let secret = BASE64_STANDARD.decode(secret).unwrap();

    let mut iv = vec![0u8; 16];
    thread_rng().fill(iv.as_mut_slice());

    let ciphertext = cbc::encrypt(&secret, &KEY, <&[u8; 16]>::try_from(iv.as_slice()).unwrap());

    iv.extend(ciphertext);
    iv
}

fn oracle(ciphertext: &[u8]) -> bool {
    cbc::decrypt(
        &ciphertext[16..],
        &KEY,
        <&[u8; 16]>::try_from(&ciphertext[..16]).unwrap(),
    )
    .is_ok()
}

fn attack(secret: &[u8]) -> Vec<u8> {
    let mut plaintext = b"".to_vec();
    let mut secret = secret.to_vec();

    for _ in 0..secret.len() / 16 - 1 {
        // Loop to bruteforce one block
        for i in 1..=16 {
            let offset = secret.len() - 16 - i;
            let initial_byte = secret[offset];

            let mut secret = secret.to_vec();

            // Fix remaining bytes of the padding
            for j in 1..i {
                secret[offset + j] = i as u8 ^ plaintext[j - 1] ^ secret[offset + j];
            }

            for k in 0..=255u8 {
                secret[offset] = k;

                if oracle(&secret) {
                    // Make sure this is the padding we're looking for
                    // See https://crypto.stackexchange.com/questions/40800/is-the-padding-oracle-attack-deterministic

                    if offset % 16 == 0 || {
                        let mut secret = secret.clone();
                        secret[offset - 1] = !secret[offset - 1];
                        oracle(&secret)
                    } {
                        plaintext.insert(0, initial_byte ^ k ^ i as u8);
                        break;
                    };
                }
            }
        }

        // Cut the last block
        secret.truncate(secret.len() - 16);
    }

    plaintext
}

fn main() {
    let secret = get_secret();

    let mut secret = attack(&secret);

    let _ = pkcs7::unpad(&mut secret);

    println!("{:x?}", String::from_utf8_lossy(&secret));
}
