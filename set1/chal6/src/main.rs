fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test1 = b"this is a test";
    let test2 = b"wokka wokka!!!";

    println!("{}", get_distance(test1, test2));
    Ok(())
}

fn get_distance(x: &[u8], y: &[u8]) -> u32 {
    // Sum the hamming distance of each bytes
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
                    res as u32
                })
                .sum::<u32>()
        })
        .sum()
}
