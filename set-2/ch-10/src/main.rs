use my_cryptopals_lib::aes;
use my_cryptopals_lib::read_long_bytes_from_stdin;
use my_cryptopals_lib::xor;

fn main() {
    let data = read_long_bytes_from_stdin().unwrap();
    let key = b"YELLOW SUBMARINE";
    let iv = &[0u8; 16];

    let mut prev_cipher: &[u8] = iv;
    let mut decoded = Vec::new();
    for i in (0..data.len()).step_by(16) {
        let block = &data[i..i+16];
        let decrypted_block = aes::decrypt_128_ecb_block(block, key);
        let mut decrypted_block = xor::decode(&decrypted_block, prev_cipher);
        prev_cipher = block;
        decoded.append(&mut decrypted_block);
    }
    let text = std::str::from_utf8(&decoded).unwrap();
    println!("{}", text);
}
