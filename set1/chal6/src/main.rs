use anyhow::Result;
use base64::prelude::*;
use itertools::Itertools;
use rayon::prelude::*;

struct KeysizeValue {
    keysize: usize,
    value: f64,
}

impl PartialEq for KeysizeValue {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for KeysizeValue {}

impl PartialOrd for KeysizeValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl Ord for KeysizeValue {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("got NaN of Infinity")
    }
}

fn find_key(ciphertext: &[u8], keysize: usize) -> Vec<u8> {
    let mut matrix = vec![vec![0u8; ciphertext.len() / keysize]; keysize];
    let mut key = vec![0u8; keysize];

    // Transpose the blocks
    for (i, chunk) in ciphertext.chunks_exact(keysize).enumerate() {
        for (j, val) in chunk.into_iter().enumerate() {
            matrix[j][i] = *val;
        }
    }

    for (i, c) in matrix.into_iter().enumerate() {
        key[i] = single_byte_xor::brute_force(&c).unwrap().key;
    }

    key
}

fn decrypt_ciphertext(ciphertext: &[u8], key: &[u8]) -> Vec<u8> {
    let mut plaintext = ciphertext.to_vec();

    for i in 0..plaintext.len() {
        plaintext[i] ^= key[i % key.len()];
    }

    plaintext
}

fn get_ciphertext() -> Result<Vec<u8>> {
    Ok(BASE64_STANDARD.decode(include_str!("../ciphertext.txt").replace("\n", ""))?)
}

fn find_keysize(ciphertext: &[u8], num_results: usize) -> Vec<usize> {
    (2..=40usize)
        .map(|keysize| KeysizeValue {
            keysize,
            value: {
                // Average the Hamming distance between each pair
                let accumulator: usize = ciphertext
                    .chunks_exact(keysize)
                    .tuple_windows()
                    .par_bridge()
                    .map(|(a, b)| get_distance(a, b))
                    .sum();

                let mut accumulator = accumulator as f64 / (ciphertext.len() / keysize) as f64;
                accumulator /= keysize as f64;
                accumulator
            },
        })
        .k_smallest(num_results)
        .map(|k| k.keysize)
        .collect()
}

fn get_distance(x: &[u8], y: &[u8]) -> usize {
    // Sum the Hamming distance of each bytes
    x.iter()
        .zip(y.iter())
        .map(|(x, y)| {
            // Sum the hamming distance of each bits in the byte
            let mut x = *x;
            let mut y = *y;
            (0..8)
                .into_iter()
                .map(|_| {
                    let res = (x & 1) ^ (y & 1);
                    x >>= 1;
                    y >>= 1;
                    res as usize
                })
                .sum::<usize>()
        })
        .sum()
}

#[test]
fn test_get_distance() {
    assert_eq!(get_distance(b"this is a test", b"wokka wokka!!!"), 37);
}

fn main() -> Result<()> {
    let ciphertext = get_ciphertext()?;
    let keysizes = find_keysize(&ciphertext, 1);

    for keysize in keysizes {
        let key = find_key(&ciphertext, keysize);
        let plaintext = decrypt_ciphertext(&ciphertext, &key);

        match String::from_utf8(plaintext) {
            Ok(p) => println!("Keysize {}: {}", keysize, p),
            _ => {
                println! {"Invalid UTF-8 received!"}
            }
        }
    }

    Ok(())
}
