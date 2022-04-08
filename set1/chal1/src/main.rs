use base64;
use hex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let raw_data = hex::decode(data)?;
    let base64_encoded = base64::encode(&raw_data);

    println!("The encoded data is: {base64_encoded}");

    Ok(())
}
