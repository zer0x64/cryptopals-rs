fn main() {
    let bin1 = from_hex("1c0111001f010100061a024b53535009181c");
    let bin2 = from_hex("686974207468652062756c6c277320657965");

    let mut result = Vec::new();

    for i in 0..bin1.len() {
        result.push(bin1[i] ^ bin2[i]);
    }

    let result = String::from_utf8(result).unwrap();

    println!("{}", result);

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