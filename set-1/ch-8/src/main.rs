use my_cryptopals_lib::{block_cipher, print_hex_bytes, read_hex_bytes_from_stdin};

fn main() -> Result<(), String> {
    loop {
        let data = read_hex_bytes_from_stdin()?;
        if data.len() == 0 {
            break;
        }
        if block_cipher::is_ecb_encrypted(&data) {
            print!("Found data likely encoded by ECB cipher: ");
            print_hex_bytes(&data);
        }
    }
    Ok(())
}
