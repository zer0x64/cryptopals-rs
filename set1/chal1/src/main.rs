use std::string::String;

fn main() {
    let base64chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bin = from_hex("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");

    let mut res = Vec::new();
    for i in 0..(bin.len() / 3 + 1){
        if i == bin.len() / 3 {
            // Finalize
            println!("Time to finalize...");
        }
        else {
            let current_work = &bin[i*3.. i*3 + 3];
            // First char..
            let mut index = (current_work[0] & 0xFC) >> 2;
            res.push(base64chars.chars().nth(index as usize).unwrap());
            index = ((current_work[0] & 0x03) << 4) | ((current_work[1] & 0xF0) >> 4);
            res.push(base64chars.chars().nth(index as usize).unwrap());
            index = ((current_work[1] & 0x0F) << 2) | ((current_work[2] & 0xFC) >> 6);
            res.push(base64chars.chars().nth(index as usize).unwrap());
            index = current_work[2] & 0x3F;
            res.push(base64chars.chars().nth(index as usize).unwrap());
        }

    }

    let result_str: String = res.into_iter().collect();
    println!("{:?}", result_str);

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