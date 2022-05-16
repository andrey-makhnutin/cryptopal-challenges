use std::io;
use std::io::Write;
use my_cryptopals_lib::{
    read_long_bytes_from_stdin,
    bytes_to_str_or_hex,
    aes,
};

fn main() -> Result<(), String> {
    let mut data = read_long_bytes_from_stdin()?;
    let len = read_int("Enter block length: ");
    aes::pad_pkcs7(&mut data, len);
    println!("{}", bytes_to_str_or_hex(&data));
    Ok(())
}

fn read_int(prompt: &str) -> usize {
    print!("{}", prompt);
    let stdin = io::stdin();
    io::stdout().flush().unwrap();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let line = line.trim();
    usize::from_str_radix(&line, 10).unwrap()
}
