use my_cryptopals_lib::{
    read_long_string_from_stdin,
    print_hex_bytes,
    xor
};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_long_string_from_stdin();
    print!("Enter key string: ");
    std::io::stdout().flush()?;
    let key = read_key()?;

    let encrypted = xor::decode(&data, &key);
    print_hex_bytes(&encrypted);

    Ok(())
}

fn read_key() -> std::io::Result<Vec<u8>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    let buf = buf.trim();
    Ok(Vec::from(buf.as_bytes()))
}
