use base64::prelude::*;

fn main() {
    let key = b"YELLOW SUBMARINE";
    let nonce = [0u8; 8];

    let ciphertext = BASE64_STANDARD
        .decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==")
        .unwrap();

    let plaintext = ctr::encrypt_decrypt(&ciphertext, key, &nonce);

    println!("{}", String::from_utf8_lossy(&plaintext));
}
