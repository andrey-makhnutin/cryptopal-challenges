use my_cryptopals_lib::aes;
use my_cryptopals_lib::block_cipher;
use rand::prelude::*;

fn main() -> Result<(), String> {
    let mut oracle_right = 0;

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let is_ecb: bool = rng.gen();
        let guessed_ecb = block_cipher::ecb_oracle(|data| encrypt(data, is_ecb));
        if guessed_ecb == is_ecb {
            oracle_right += 1;
        }
    }
    println!("ECB oracle was right {} times", oracle_right);

    Ok(())
}

fn encrypt(data: &[u8], is_ecb: bool) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key: [u8; 16] = rand::random();
    let prepend_len: usize = rng.gen_range(5..=10);
    let append_len: usize = rng.gen_range(5..=10);
    let mut padded_data = vec![0u8; data.len() + prepend_len + append_len];
    rng.fill_bytes(&mut padded_data[0..prepend_len]);
    let mut cursor = prepend_len;
    padded_data[cursor..cursor + data.len()].copy_from_slice(data);
    cursor += data.len();
    rng.fill_bytes(&mut padded_data[cursor..cursor + append_len]);

    match is_ecb {
        true => aes::encrypt_128_ecb(&padded_data, &key),
        false => {
            let iv: [u8; 16] = rand::random();
            aes::encrypt_128_cbc(&padded_data, &iv, &key)
        }
    }
}
