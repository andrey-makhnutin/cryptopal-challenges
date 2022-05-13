use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;

pub fn encrypt_128_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ctx = CipherCtx::new().unwrap();
    ctx.encrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    let mut out = Vec::new();
    ctx.cipher_update_vec(data, &mut out).unwrap();
    ctx.cipher_final_vec(&mut out).unwrap();

    out
}

pub fn encrypt_128_ecb_block(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ctx = CipherCtx::new().unwrap();
    ctx.encrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    let mut out = Vec::new();
    ctx.set_padding(false);
    ctx.cipher_update_vec(data, &mut out).unwrap();
    ctx.cipher_final_vec(&mut out).unwrap();

    out
}

pub fn decrypt_128_ecb(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    let mut out = Vec::new();
    ctx.cipher_update_vec(data, &mut out).unwrap();
    ctx.cipher_final_vec(&mut out).unwrap();

    out
}

pub fn decrypt_128_ecb_block(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    let mut out = Vec::new();
    ctx.set_padding(false);
    ctx.cipher_update_vec(data, &mut out).unwrap();

    out
}
