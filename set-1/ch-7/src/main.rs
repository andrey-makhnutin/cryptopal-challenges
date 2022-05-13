use my_cryptopals_lib::read_long_bytes_from_stdin;
use my_cryptopals_lib::aes::decrypt_128_ecb;

fn main() -> Result<(), String> {
    let data = read_long_bytes_from_stdin()?;
    let key = b"YELLOW SUBMARINE";

    let mut plaintext = decrypt_128_ecb(&data, key);
    let text = std::str::from_utf8(&plaintext).or(Err("Failed to decode as utf8"))?;
    println!("{}", text);

    Ok(())
}
