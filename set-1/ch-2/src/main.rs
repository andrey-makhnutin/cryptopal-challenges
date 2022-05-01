use my_cryptopals_lib::read_hex_bytes_from_stdin;

fn main() -> Result<(), String> {
    println!("Enter data hex bytes");
    let data_bytes = read_hex_bytes_from_stdin()?;
    println!("Enter key hex bytes");
    let key_bytes = read_hex_bytes_from_stdin()?;
    try_print_utf8(&key_bytes);
    let result_bytes = xor_bytes(&data_bytes, &key_bytes)?;
    print_hex_bytes(&result_bytes);
    try_print_utf8(&result_bytes);
    Ok(())
}

fn xor_bytes(data_bytes: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let mut res = Vec::with_capacity(data_bytes.len());
    if data_bytes.len() != key_bytes.len() {
        return Err("Data bytes and key bytes must be the same length".into());
    }
    for (d, k) in data_bytes.iter().zip(key_bytes) {
        res.push(d ^ k);
    }
    Ok(res)
}

fn print_hex_bytes(bytes: &[u8]) {
    for b in bytes {
        print!(
            "{}{}",
            char::from_digit((b >> 4) as u32, 16).unwrap(),
            char::from_digit((b & 0b1111) as u32, 16).unwrap()
        );
    }
    println!()
}

fn try_print_utf8(bytes: &[u8]) {
    let res = std::str::from_utf8(bytes);
    if let Ok(str) = res {
        println!("Can be decoded as utf8: {}", str);
    }
}
