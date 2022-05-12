use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    try_print_utf8,
    print_hex_bytes,
    xor
};

fn main() -> Result<(), String> {
    println!("Enter data hex bytes");
    let data_bytes = read_hex_bytes_from_stdin()?;
    println!("Enter key hex bytes");
    let key_bytes = read_hex_bytes_from_stdin()?;
    try_print_utf8(&key_bytes);
    let result_bytes = xor::decode(&data_bytes, &key_bytes);
    println!("Decoded bytes are");
    print_hex_bytes(&result_bytes);
    try_print_utf8(&result_bytes);
    Ok(())
}
