use my_cryptopals_lib::{
    read_long_bytes_from_stdin,
    bytes_to_str_or_hex,
};
use my_cryptopals_lib::aes::decrypt_128_ecb;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_long_bytes_from_stdin()?;
    let key = b"YELLOW SUBMARINE";

    let plaintext = decrypt_128_ecb(&data, key)?;
    println!("{}", bytes_to_str_or_hex(&plaintext));

    Ok(())
}
