use anyhow::Result;
use itertools::Itertools;

fn get_ciphertexts() -> Result<Vec<Vec<u8>>> {
    let ciphertexts: std::result::Result<Vec<Vec<u8>>, hex::FromHexError> =
        include_str!("../ciphertexts.txt")
            .lines()
            .map(hex::decode)
            .collect();

    Ok(ciphertexts?)
}

fn main() -> Result<()> {
    let ciphertexts: Vec<Vec<u8>> = get_ciphertexts()?;

    for (i, c) in ciphertexts.into_iter().enumerate() {
        for (a, b) in c.chunks_exact(16).tuple_combinations() {
            if a == b {
                println!("Line {} is in ECB: {}", i + 1, hex::encode(&c));
                return Ok(());
            }
        }
    }
    println!("Hello, world!");

    Ok(())
}
