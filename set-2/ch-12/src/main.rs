use my_cryptopals_lib::aes;
use my_cryptopals_lib::block_cipher;
use my_cryptopals_lib::bytes_to_str_or_hex;
use my_cryptopals_lib::read_long_bytes_from_stdin;

fn main() -> Result<(), String> {
    let key: [u8; 16] = rand::random();
    let secret_data = read_long_bytes_from_stdin()?;

    let decrypted_secret =
        block_cipher::decrypt_ebc_appended_string(|data| encrypt(data, &secret_data, &key))?;
    println!(
        "Decrypted secret bytes as {}",
        bytes_to_str_or_hex(&decrypted_secret)
    );
    Ok(())
}

fn encrypt(data: &[u8], secret_data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut padded_data = vec![0u8; data.len() + secret_data.len()];
    padded_data[0..data.len()].copy_from_slice(data);
    padded_data[data.len()..data.len() + secret_data.len()].copy_from_slice(secret_data);
    aes::encrypt_128_ecb(&padded_data, &key)
}
