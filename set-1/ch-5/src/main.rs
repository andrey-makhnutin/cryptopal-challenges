use my_cryptopals_lib::{
    read_long_string_from_stdin,
    print_hex_bytes,
};
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_long_string_from_stdin();
    print!("Enter key string: ");
    std::io::stdout().flush()?;
    let key = read_key()?;

    let encrypted = encrypt_rolling_xor(&data, &key);
    print_hex_bytes(&encrypted);

    Ok(())
}


fn read_key() -> std::io::Result<Vec<u8>> {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;
    let buf = buf.trim();
    Ok(Vec::from(buf.as_bytes()))
}

fn encrypt_rolling_xor(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(data.len());

    for i in 0..data.len() {
        let k = key[i % key.len()];
        out.push(data[i] ^ k);
    }

    out
}
