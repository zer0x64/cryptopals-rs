use rayon::prelude::*;
use std::collections::HashMap;

lazy_static::lazy_static! {
    // This is a map of the character frequency of ever english character
    pub static ref ENGLISH_CHARACTER_FREQUENCIES: HashMap<char, f32> = {
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
pub struct Res {
    pub plaintext: String,
    pub key: u8,
    pub score: f32,
}

pub fn brute_force(data: &[u8]) -> Option<Res> {
    (0..=0xFF)
        .into_par_iter()
        .map(|k| {
            // XOR bytes and filter out non-ascii
            match String::from_utf8(xor_bytes(data, k)) {
                Ok(p) => Some((k, p)),
                Err(_) => None,
            }
        })
        .flatten()
        .map(|(k, p)| {
            // Get the score
            let score = get_score(&p);

            Res {
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

pub fn xor_bytes(data: &[u8], key: u8) -> Vec<u8> {
    // XOR every byte of the array with the key
    data.iter().map(|x| *x ^ key).collect()
}

pub fn get_score(data: &str) -> f32 {
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
