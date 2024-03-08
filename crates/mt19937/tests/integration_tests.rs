use rand::prelude::*;

use mt19937::Mt19937Rng;

#[test]
fn test_mt19937_u32_seed() {
    let rng = Mt19937Rng::seed_from_u64(1234);

    let expected: Vec<u32> = include_str!("test_data/u32_seed.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    assert_eq!(&expected, &rng.state);
}

#[test]
fn test_mt19937_array_seed() {
    let rng = Mt19937Rng::from_seed([
        0x00, 0x00, 0x01, 0x23, 0x00, 0x00, 0x02, 0x34, 0x00, 0x00, 0x03, 0x45, 0x00, 0x00, 0x04,
        0x56,
    ]);

    let expected: Vec<u32> = include_str!("test_data/array_seed.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect();

    assert_eq!(&expected, &rng.state);
}

#[test]
fn test_mt19937_u32_seed_gen() {
    let mut rng = Mt19937Rng::seed_from_u64(1234);

    for n in include_str!("test_data/u32_seed_gen.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
    {
        assert_eq!(n, rng.next_u32());
    }
}

#[test]
fn test_mt19937_array_seed_gen() {
    let mut rng = Mt19937Rng::from_seed([
        0x00, 0x00, 0x01, 0x23, 0x00, 0x00, 0x02, 0x34, 0x00, 0x00, 0x03, 0x45, 0x00, 0x00, 0x04,
        0x56,
    ]);

    for n in include_str!("test_data/array_seed_gen.txt")
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
    {
        assert_eq!(n, rng.next_u32());
    }
}
