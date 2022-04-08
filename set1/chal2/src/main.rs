fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = hex_literal::hex!("1c0111001f010100061a024b53535009181c");
    let key = hex_literal::hex!("686974207468652062756c6c277320657965");

    let result = xor_bytes(&data, &key);
    let result = hex::encode(result);

    println!("Result is: {result}");
    Ok(())
}

fn xor_bytes(data: &[u8], key: &[u8]) -> Vec<u8> {
    data.iter().zip(key.iter()).map(|(x, y)| *x ^ *y).collect()
}
