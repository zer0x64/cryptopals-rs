extern crate hex;
use std::string::String;
use std::str;
//use hex;

fn main() {
    let base64chars = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/");
    let bin = hex::decode("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").unwrap();

    let mut res = Vec::new();
    for i in 0..(bin.len() / 3 + 1){
        if i == bin.len() / 3 {
            // Finalize
            println!("Time to finalize...");
        }
        else {
            let currentWork = &bin[i*3.. i*3 + 3];
            // First char..
            let mut index = (currentWork[0] & 0xFC) >> 2;
            res.push(base64chars.chars().nth(index as usize).unwrap());
            let mut index = ((currentWork[0] & 0x03) << 4) | ((currentWork[1] & 0xF0) >> 4);
            res.push(base64chars.chars().nth(index as usize).unwrap());
            let mut index = ((currentWork[1] & 0x0F) << 2) | ((currentWork[2] & 0xFC) >> 6);
            res.push(base64chars.chars().nth(index as usize).unwrap());
            let mut index = (currentWork[2] & 0x3F);
            res.push(base64chars.chars().nth(index as usize).unwrap());
        }

    }

    let result_str:String = res.into_iter().collect();
    println!("{:?}", result_str);

}

