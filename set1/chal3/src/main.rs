use std::collections::HashMap;

fn main() {
    let char_frequency = char_frequency();
    let ciphertext = from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let mut best_key = 0u8;
    let mut best_score = 0u32;

    for key in 0..255u8 {
        let mut score = 0u32;
        for i in 0..ciphertext.len() {
            score += char_frequency.get(&(ciphertext[i] ^ key)).unwrap_or(&0).clone() as u32;
        }

        if score > best_score {
            best_key = key;
            best_score = score;
        }
    }

    let mut result = Vec::new();

    for i in 0..ciphertext.len() {
        result.push(ciphertext[i] ^ best_key);
    }

    let result = String::from_utf8(result).unwrap();

    println!("Best key: {}\nScore: {}\nPlaintext: {}", best_key, best_score, result);

}

fn char_frequency() -> HashMap<u8, u8> {
    let mut chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".bytes().into_iter();
    let mut map = HashMap::new();
    map.insert(chars.next().unwrap(), 82);        //A
    map.insert(chars.next().unwrap(), 15);        //B
    map.insert(chars.next().unwrap(), 27);        //C
    map.insert(chars.next().unwrap(), 42);        //D
    map.insert(chars.next().unwrap(), 127);       //E
    map.insert(chars.next().unwrap(), 22);        //F
    map.insert(chars.next().unwrap(), 20);        //G
    map.insert(chars.next().unwrap(), 61);        //H
    map.insert(chars.next().unwrap(), 70);        //I
    map.insert(chars.next().unwrap(), 2);         //J
    map.insert(chars.next().unwrap(), 8);         //K
    map.insert(chars.next().unwrap(), 40);        //L
    map.insert(chars.next().unwrap(), 24);        //M
    map.insert(chars.next().unwrap(), 67);        //N
    map.insert(chars.next().unwrap(), 75);        //O
    map.insert(chars.next().unwrap(), 19);        //P
    map.insert(chars.next().unwrap(), 1);         //Q
    map.insert(chars.next().unwrap(), 60);        //R
    map.insert(chars.next().unwrap(), 63);        //S
    map.insert(chars.next().unwrap(), 91);        //T
    map.insert(chars.next().unwrap(), 28);        //U
    map.insert(chars.next().unwrap(), 10);        //V
    map.insert(chars.next().unwrap(), 24);        //W
    map.insert(chars.next().unwrap(), 2);         //X
    map.insert(chars.next().unwrap(), 20);        //Y
    map.insert(chars.next().unwrap(), 1);         //Z

    map.insert(chars.next().unwrap(), 82);        //a
    map.insert(chars.next().unwrap(), 15);        //b
    map.insert(chars.next().unwrap(), 27);        //c
    map.insert(chars.next().unwrap(), 42);        //d
    map.insert(chars.next().unwrap(), 127);       //e
    map.insert(chars.next().unwrap(), 22);        //f
    map.insert(chars.next().unwrap(), 20);        //g
    map.insert(chars.next().unwrap(), 61);        //h
    map.insert(chars.next().unwrap(), 70);        //i
    map.insert(chars.next().unwrap(), 2);         //j
    map.insert(chars.next().unwrap(), 8);         //k
    map.insert(chars.next().unwrap(), 40);        //l
    map.insert(chars.next().unwrap(), 24);        //m
    map.insert(chars.next().unwrap(), 67);        //n
    map.insert(chars.next().unwrap(), 75);        //o
    map.insert(chars.next().unwrap(), 19);        //p
    map.insert(chars.next().unwrap(), 1);         //q
    map.insert(chars.next().unwrap(), 60);        //r
    map.insert(chars.next().unwrap(), 63);        //s
    map.insert(chars.next().unwrap(), 91);        //t
    map.insert(chars.next().unwrap(), 28);        //u
    map.insert(chars.next().unwrap(), 10);        //v
    map.insert(chars.next().unwrap(), 24);        //w
    map.insert(chars.next().unwrap(), 2);         //x
    map.insert(chars.next().unwrap(), 20);        //y
    map.insert(chars.next().unwrap(), 1);         //z

    map
}

fn from_hex(input: &str) -> Vec<u8>{
    let hex_chars = "0123456789abcdef".as_bytes();
    let input = input.as_bytes();

    let mut result = Vec::new();

    for i in 0..input.len()/2 {
        let mut current_byte = hex_chars.iter().position(|&x| x == input[2 * i]).unwrap() as u8;
        current_byte <<= 4;
        current_byte |= hex_chars.iter().position(|&x| x == input[2 * i + 1]).unwrap() as u8;

        result.push(current_byte);
    };

    return result;
}