use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, PartialEq, Eq, Error)]
pub struct InvalidPadding;

impl Display for InvalidPadding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("InvalidPadding")
    }
}

pub fn pad(data: &mut Vec<u8>, block_size: u8) {
    let pad_length = block_size - (data.len() % block_size as usize) as u8;

    data.resize(data.len() + pad_length as usize, pad_length as u8);
}

pub fn unpad(data: &mut Vec<u8>) -> Result<(), InvalidPadding> {
    let pad_length = data[data.len() - 1];

    // Validate padding
    if pad_length == 0 || pad_length as usize >= data.len() {
        return Err(InvalidPadding);
    }

    for x in data[(data.len() - pad_length as usize)..].iter() {
        if *x != pad_length {
            return Err(InvalidPadding);
        }
    }

    // Unpad
    data.resize(data.len() - data[data.len() - 1] as usize, pad_length);

    Ok(())
}

#[test]
fn test_pad() {
    let mut plaintext = b"YELLOW SUBMARINE".to_vec();
    pad(&mut plaintext, 20);

    assert_eq!(&plaintext, b"YELLOW SUBMARINE\x04\x04\x04\x04");

    pad(&mut plaintext, 20);

    assert_eq!(&plaintext,  b"YELLOW SUBMARINE\x04\x04\x04\x04\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14");
}

#[test]
fn test_unpad() {
    let mut plaintext = b"YELLOW SUBMARINE\x04\x04\x04\x04\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14\x14".to_vec();
    unpad(&mut plaintext).expect("Padding invalid!");

    assert_eq!(&plaintext, b"YELLOW SUBMARINE\x04\x04\x04\x04");

    unpad(&mut plaintext).expect("Padding invalid!");

    assert_eq!(&plaintext, b"YELLOW SUBMARINE");

    let result = unpad(&mut plaintext);

    assert_eq!(result, Err(InvalidPadding));
}
