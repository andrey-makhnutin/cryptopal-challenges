use my_cryptopals_lib::aes;
use my_cryptopals_lib::{
    read_long_bytes_from_stdin,
    bytes_to_str_or_hex,
};

fn main() {
    let data = read_long_bytes_from_stdin().unwrap();
    let key = b"YELLOW SUBMARINE";
    let iv = &[0u8; 16];

    let decoded = aes::decrypt_128_cbc(&data, iv, key).unwrap();
    println!("{}", bytes_to_str_or_hex(&decoded));
}
