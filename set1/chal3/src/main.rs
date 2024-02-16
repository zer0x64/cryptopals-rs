fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data =
        hex_literal::hex!("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");

    let result = single_byte_xor::brute_force(&data).ok_or("No results!")?;

    println!(
        "This was encoded using key: {:X}\nIt got the following score: {}\nResult string is: {}",
        result.key, result.score, result.plaintext
    );
    Ok(())
}
