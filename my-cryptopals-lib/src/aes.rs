use openssl::cipher::Cipher;
use openssl::cipher_ctx::CipherCtx;

#[derive(Debug)]
pub struct AesEncryptError(String);
impl std::fmt::Display for AesEncryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for AesEncryptError {}

pub fn encrypt_128_ecb(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    let mut padded_data = data.to_vec();
    pad_pkcs7(&mut padded_data, 16);
    let mut ctx = CipherCtx::new().unwrap();
    ctx.encrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    ctx.set_padding(false);
    let mut out = vec![0u8; padded_data.len() + 16];
    for i in (0..padded_data.len()).step_by(16) {
        let in_block = &padded_data[i..i + 16];
        let out_block = &mut out[i..i + 32];
        ctx.cipher_update(in_block, Some(out_block)).unwrap();
    }
    out.truncate(out.len() - 16);

    out
}

pub fn encrypt_128_cbc(data: &[u8], iv: &[u8; 16], key: &[u8; 16]) -> Vec<u8> {
    let mut padded_data = data.to_vec();
    pad_pkcs7(&mut padded_data, 16);
    let mut ctx = CipherCtx::new().unwrap();
    ctx.encrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    ctx.set_padding(false);
    let mut prev_block = iv as &[u8];
    let mut out = vec![0u8; padded_data.len() + 16];
    let mut temp_block = [0; 16];
    for i in (0..padded_data.len()).step_by(16) {
        let in_block = &padded_data[i..i + 16];
        crate::xor::encode_into(in_block, prev_block, &mut temp_block);
        let out_block = &mut out[i..i + 32];
        ctx.cipher_update(&temp_block, Some(out_block)).unwrap();
        prev_block = &out[i..i + 16];
    }
    out.truncate(out.len() - 16);

    out
}

#[derive(Debug)]
pub struct AesDecryptError(String);
impl std::fmt::Display for AesDecryptError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for AesDecryptError {}

pub fn decrypt_128_ecb(data: &[u8], key: &[u8; 16]) -> Result<Vec<u8>, AesDecryptError> {
    if data.len() % 16 != 0 {
        return Err(AesDecryptError(format!(
            "Encrypted data length is not a multiple of 16: {}",
            data.len()
        )));
    }
    if key.len() != 16 {
        return Err(AesDecryptError(format!(
            "Key length must be 16, got {}",
            key.len()
        )));
    }
    let mut out = vec![0u8; data.len() + 16];
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    ctx.set_padding(false);
    for i in (0..data.len()).step_by(16) {
        let in_block = &data[i..i + 16];
        let out_block = &mut out[i..i + 32];
        ctx.cipher_update(in_block, Some(out_block)).unwrap();
    }
    out.truncate(out.len() - 16);
    unpad_pkcs7(&mut out, 16)?;

    Ok(out)
}

pub fn decrypt_128_cbc(
    data: &[u8],
    iv: &[u8; 16],
    key: &[u8; 16],
) -> Result<Vec<u8>, AesDecryptError> {
    if data.len() % 16 != 0 {
        return Err(AesDecryptError(format!(
            "Encrypted data length is not a multiple of 16: {}",
            data.len()
        )));
    }
    if key.len() != 16 {
        return Err(AesDecryptError(format!(
            "Key length must be 16, got {}",
            key.len()
        )));
    }
    let mut out = vec![0u8; data.len()];
    let mut ctx = CipherCtx::new().unwrap();
    ctx.decrypt_init(Some(Cipher::aes_128_ecb()), Some(key), None)
        .unwrap();
    ctx.set_padding(false);
    let mut prev_block = iv as &[u8];
    let mut temp_block = [0u8; 32];
    for i in (0..data.len()).step_by(16) {
        let in_block = &data[i..i + 16];
        ctx.cipher_update(&in_block, Some(&mut temp_block)).unwrap();
        let out_block = &mut out[i..i + 16];
        crate::xor::encode_into(&temp_block[..16], prev_block, out_block);
        prev_block = in_block;
    }
    unpad_pkcs7(&mut out, 16)?;

    Ok(out)
}

pub fn pad_pkcs7(data: &mut Vec<u8>, block_len: usize) {
    if block_len > 255 {
        panic!("Block size can't be bigger than 255 bytes");
    }
    let pad_len = block_len - data.len() % block_len;
    let pad_byte = pad_len as u8;
    let mut pad_vec = vec![pad_byte; pad_len];
    data.append(&mut pad_vec);
}

pub fn unpad_pkcs7(data: &mut Vec<u8>, block_len: usize) -> Result<(), AesDecryptError> {
    if data.len() == 0 {
        return Err(AesDecryptError("Empty data can't be unpadded".to_owned()));
    }
    let pad_len = data[data.len() - 1] as usize;
    if pad_len > block_len {
        return Err(AesDecryptError(format!(
            "Found padding length that is bigger than block size: {} > {}",
            pad_len, block_len
        )));
    }
    if pad_len > data.len() {
        return Err(AesDecryptError(format!(
            "Padding length is bigger than data len: {} > {}",
            pad_len,
            data.len()
        )));
    }
    if pad_len == 0 {
        return Err(AesDecryptError(format!("Found zero padding length")));
    }
    let padding_slice = &data[data.len() - pad_len..];
    let expected_padding = vec![pad_len as u8; pad_len];
    if padding_slice != expected_padding {
        return Err(AesDecryptError(format!(
            "Found invalid padding: {:?}",
            padding_slice
        )));
    }
    data.truncate(data.len() - pad_len);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_ecb_test() {
        let data1 = b"HELLO";
        let key = b"YELLOW SUBMARINE";
        let encrypted = encrypt_128_ecb(data1, key);
        let decrypted = decrypt_128_ecb(&encrypted, key).unwrap();
        assert_eq!(&decrypted, data1);

        let data2 = b"YELLOW SUBMARINE";
        let key = b"YELLOW SUBMARINE";
        let encrypted = encrypt_128_ecb(data2, key);
        let decrypted = decrypt_128_ecb(&encrypted, key).unwrap();
        assert_eq!(&decrypted, data2);

        let data3 = b"";
        let key = b"YELLOW SUBMARINE";
        let encrypted = encrypt_128_ecb(data3, key);
        let decrypted = decrypt_128_ecb(&encrypted, key).unwrap();
        assert_eq!(&decrypted, data3);
    }

    #[test]
    fn encrypt_cbc_test() {
        let data1 = b"HELLO";
        let key = b"YELLOW SUBMARINE";
        let iv = [0u8; 16];
        let encrypted = encrypt_128_cbc(data1, &iv, key);
        let decrypted = decrypt_128_cbc(&encrypted, &iv, key).unwrap();
        assert_eq!(&decrypted, data1);

        let data2 = b"YELLOW SUBMARINE";
        let key = b"YELLOW SUBMARINE";
        let encrypted = encrypt_128_cbc(data2, &iv, key);
        let decrypted = decrypt_128_cbc(&encrypted, &iv, key).unwrap();
        assert_eq!(&decrypted, data2);

        let data3 = b"";
        let key = b"YELLOW SUBMARINE";
        let encrypted = encrypt_128_cbc(data3, &iv, key);
        let decrypted = decrypt_128_cbc(&encrypted, &iv, key).unwrap();
        assert_eq!(&decrypted, data3);
    }
}
