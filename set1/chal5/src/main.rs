fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plaintext = b"Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    let key = b"ICE";

    let ciphertext = encrypt(plaintext, key);

    let ciphertext = hex::encode(ciphertext);

    println!("The ciphertext is: {ciphertext}");
    Ok(())
}

fn encrypt(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter()
        .enumerate()
        .map(|(i, d)| *d ^ key[i % key.len()])
        .collect()
}
