use std::str::FromStr;

use rand::prelude::*;

lazy_static::lazy_static! {
    static ref KEY: [u8; 16] = {
        let mut key = [0u8; 16];
        thread_rng().fill(&mut key);

        key
    };
}

/// Encoding for serde_urlencoded is too strong so we cannot use it :(
#[derive(Default, Debug)]
struct Profile {
    email: String,
    uid: usize,
    role: String,
}

impl ToString for Profile {
    fn to_string(&self) -> String {
        "email=".to_string()
            + &self.email.replace("&", "").replace("=", "")
            + "&uid="
            + &self.uid.to_string()
            + "&role="
            + &self.role.replace("&", "").replace("=", "")
    }
}

impl FromStr for Profile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut profile = Profile::default();

        for s in s.split("&") {
            let mut s = s.split("=");
            match s.next() {
                Some("email") => {
                    if let Some(s) = s.next() {
                        profile.email = s.to_string();
                    }
                }
                Some("uid") => {
                    if let Some(s) = s.next() {
                        match s.parse::<usize>() {
                            Ok(uid) => profile.uid = uid,
                            _ => {}
                        }
                    }
                }
                Some("role") => {
                    if let Some(s) = s.next() {
                        profile.role = s.to_string();
                    }
                }
                _ => {}
            }
        }

        Ok(profile)
    }
}

fn profile_for(email: &str) -> Vec<u8> {
    let profile = &Profile {
        email: email.to_string(),
        uid: 10,
        role: "user".to_string(),
    }
    .to_string();

    ecb::encrypt(profile.as_bytes(), &KEY)
}

fn decode_profile(ciphertext: &[u8]) -> Profile {
    let plaintext = String::from_utf8(ecb::decrypt(ciphertext, &KEY).unwrap()).unwrap();

    Profile::from_str(&plaintext).unwrap()
}

/// Here, we try to get a valid "admin\x0b\x0b..." block.
fn get_admin_block() -> Vec<u8> {
    // We finish the first block
    let mut email = "A".repeat(16 - "email=".len());

    // We append the block
    email = email + "admin" + &"\x0b".repeat(11);

    // We retrieve the block
    profile_for(&email)[16..32].to_vec()
}

fn get_ciphertext() -> Vec<u8> {
    let initial_size = profile_for("").len();
    let mut current_offset = 1;

    loop {
        let ciphertext = profile_for(&"A".repeat(current_offset));

        if ciphertext.len() > initial_size {
            break;
        }

        current_offset += 1;
    }

    profile_for(&"A".repeat(current_offset + "user".len()))
}

fn main() {
    let admin_block = get_admin_block();
    let mut ciphertext = get_ciphertext();
    let offset = ciphertext.len() - 16;

    for i in 0..16 {
        ciphertext[offset + i] = admin_block[i];
    }

    let profile = decode_profile(&ciphertext);
    println!("{:?}", profile);
}
