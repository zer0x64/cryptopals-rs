use rand::prelude::*;

lazy_static::lazy_static! {
    static ref KEY: [u8; 16] = {
        let mut key = [0u8; 16];
        thread_rng().fill(&mut key);

        key
    };
}

const PREFIX: &'static str = "comment1=cooking%20MCs;userdata=";
const SUFFIX: &'static str = ";comment2=%20like%20a%20pound%20of%20bacon";

fn encode_cookie(userdata: &str) -> Vec<u8> {
    let cookie = PREFIX.to_string() + &userdata.replace(";", "").replace("=", "") + SUFFIX;

    let mut iv = vec![0u8; 16];
    thread_rng().fill(iv.as_mut_slice());

    let ciphertext = cbc::encrypt(
        cookie.as_bytes(),
        &KEY,
        <&[u8; 16]>::try_from(iv.as_slice()).unwrap(),
    );

    iv.extend(ciphertext);
    iv
}

fn decode_cookie(cookie: &[u8]) -> bool {
    let cookie = match cbc::decrypt(
        &cookie[16..],
        &KEY,
        <&[u8; 16]>::try_from(&cookie[..16]).unwrap(),
    ) {
        Ok(c) => c,
        _ => return false,
    };

    is_sub(&cookie, b";admin=true;")
}

fn get_cookie() -> Vec<u8> {
    encode_cookie("AadminAtrueA")
}

fn modify_cookie(cookie: &mut [u8]) {
    // PREFIX is 32 bytes long, IV is 16 bytes long. We want to start at the last prefix block
    let global_offset = 32;

    let semicolon_modifier = b'A' ^ b';';
    let equal_modifier = b'A' ^ b'=';

    cookie[global_offset] ^= semicolon_modifier;
    cookie[global_offset + 6] ^= equal_modifier;
    cookie[global_offset + 11] ^= semicolon_modifier;
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
    let mut cookie = get_cookie();

    // We make sure the cookie is invalid before the forgery
    assert!(!decode_cookie(&cookie));

    modify_cookie(&mut cookie);

    // The cookie should now be valid
    assert!(decode_cookie(&cookie));

    println!("Attack succeeded!")
}
