use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data: Vec<Vec<u8>> = {
        let file = File::open("./ciphertexts.txt")?;
        let reader = BufReader::new(file);

        reader
            .lines()
            .filter_map(|x| hex::decode(&x.unwrap()).ok())
            .collect()
    };

    let result = brute_force_all(data).ok_or("No results!")?;

    println!(
        "The encoded ciphertext is: {}\n,This was encoded using key: {:X}\nIt got the following score: {}\nResult string is: {}",
        result.ciphertext, result.key, result.score, result.plaintext
    );
    Ok(())
}

lazy_static::lazy_static! {
    // This is a map of the character frequency of ever english character
    static ref ENGLISH_CHARACTER_FREQUENCIES: HashMap<char, f32> = {
        let mut m = HashMap::new();
        m.insert(' ', 0.182884);
        m.insert('a', 0.084966);
        m.insert('b', 0.020720);
        m.insert('c', 0.045388);
        m.insert('d', 0.033844);
        m.insert('e', 0.111607);
        m.insert('f', 0.018121);
        m.insert('g', 0.024705);
        m.insert('h', 0.030034);
        m.insert('i', 0.075448);
        m.insert('j', 0.001965);
        m.insert('k', 0.011016);
        m.insert('l', 0.054893);
        m.insert('m', 0.030129);
        m.insert('n', 0.066544);
        m.insert('o', 0.071635);
        m.insert('p', 0.031671);
        m.insert('q', 0.001962);
        m.insert('r', 0.075809);
        m.insert('s', 0.057351);
        m.insert('t', 0.069509);
        m.insert('u', 0.036308);
        m.insert('v', 0.010074);
        m.insert('w', 0.012899);
        m.insert('x', 0.002902);
        m.insert('y', 0.017779);
        m.insert('z', 0.002722);
        m
    };
}

#[derive(Debug, Clone)]
struct Res {
    ciphertext: String,
    plaintext: String,
    key: u8,
    score: f32,
}

fn brute_force_all(data: Vec<Vec<u8>>) -> Option<Res> {
    data.into_par_iter()
        .filter_map(|d| brute_force_single(&d))
        .min_by(|x, y| {
            // The best score is the one with the lowest deviation from the distribution(see "Fitting Quotient")
            x.score
                .partial_cmp(&y.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn brute_force_single(data: &[u8]) -> Option<Res> {
    (0..=0xFF)
        .into_par_iter()
        .filter_map(|k| {
            // XOR bytes and filter out non-ascii
            match String::from_utf8(xor_bytes(data, k)) {
                Ok(p) => Some((data, k, p)),
                Err(_) => None,
            }
        })
        .map(|(data, k, p)| {
            // Get the score
            let score = get_score(&p);

            Res {
                ciphertext: hex::encode(data),
                plaintext: p,
                key: k,
                score,
            }
        })
        .min_by(|x, y| {
            // The best score is the one with the lowest deviation from the distribution(see "Fitting Quotient")
            x.score
                .partial_cmp(&y.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

fn xor_bytes(data: &[u8], key: u8) -> Vec<u8> {
    // XOR every byte of the array with the key
    data.iter().map(|x| *x ^ key).collect()
}

fn get_score(data: &str) -> f32 {
    let mut frequencies: HashMap<char, u32> = HashMap::new();

    // We start by lower-casing the string and remove unknown characters
    let data: String = data
        .to_ascii_lowercase()
        .chars()
        .filter(|c| ENGLISH_CHARACTER_FREQUENCIES.contains_key(c))
        .collect();

    // We count every character occurence
    for c in data.chars() {
        *frequencies.entry(c).or_default() += 1;
    }

    // We convert those occurences to their proportion in the string
    let frequencies: HashMap<char, f32> = frequencies
        .into_iter()
        .map(|(k, value)| (k, value as f32 / data.len() as f32))
        .collect();

    // We calculate the fitting quotient/score of the plaintext
    let mut score = 0f32;

    for (key, value) in ENGLISH_CHARACTER_FREQUENCIES.iter() {
        score += (value - frequencies.get(key).unwrap_or(&0.0)).abs();
    }

    // We divide by the amount of known characters
    score /= ENGLISH_CHARACTER_FREQUENCIES.len() as f32;

    score
}
