use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;
use my_cryptopals_lib::read_long_bytes_from_stdin;

fn main() -> Result<(), String> {
    let cipher = Cipher::aes_128_ecb();
    let data = read_long_bytes_from_stdin()?;
    let key = b"YELLOW SUBMARINE";

    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(cipher), Some(key), None).unwrap();

    let mut plaintext = vec![];
    ctx.cipher_update_vec(&data, &mut plaintext).unwrap();
    ctx.cipher_final_vec(&mut plaintext).unwrap();

    let text = std::str::from_utf8(&plaintext).or(Err("Failed to decode as utf8"))?;
    println!("{}", text);

    Ok(())
}
