use std::collections::HashSet;

pub fn is_ecb_encrypted(data: &[u8]) -> bool {
    if data.len() % 16 != 0 {
        panic!("Found data with len not divisible by 16");
    }
    let mut blocks: HashSet<&[u8]> = HashSet::new();
    for i in (0..data.len()).step_by(16) {
        let block = &data[i..i + 16];
        if blocks.contains(block) {
            return true;
        }
        blocks.insert(block);
    }
    return false;
}

pub fn ecb_oracle<F>(encrypter: F) -> bool
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let data = ['A' as u8; 3 * 16 - 1];
    let encrypted = encrypter(&data);
    is_ecb_encrypted(&encrypted)
}

// encrypter is a function of a form
//    |data| ebc_cipher(data || some_appended_string, some_key)
// this function guesses `some_appended_string`
pub fn decrypt_ebc_appended_string<F>(encrypter: F) -> Result<Vec<u8>, String>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let block_len = guess_block_size(&encrypter).ok_or("Encrypter is not using ECB cipher")?;
    println!("Encrypter is using {} byte blocks", block_len);

    let mut known: Vec<u8> = Vec::new();
    loop {
        if let Some(b) = guess_another_byte(&encrypter, &known, block_len) {
            known.push(b);
        } else {
            break;
        }
    }

    if known.len() > 0 && known[known.len() - 1] == 1 {
        known.truncate(known.len() - 1);
    }
    Ok(known)
}

fn guess_another_byte<F>(encrypter: F, known: &[u8], block_len: usize) -> Option<u8>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    let bl_m1 = block_len - 1;
    let padding_len = bl_m1 - (known.len() % block_len);
    let mut data = vec![0u8; block_len + padding_len];
    if known.len() > bl_m1 {
        data[0..bl_m1].copy_from_slice(&known[known.len() - bl_m1..known.len()]);
    } else {
        let known_pos = bl_m1 - known.len();
        data[known_pos..known_pos + known.len()].copy_from_slice(&known);
    }
    let guessed_block_offset = ((known.len() / block_len) + 1) * block_len;
    for b in 0..=255u8 {
        data[block_len - 1] = b;
        let encrypted = encrypter(&data);
        if encrypted[0..block_len]
            == encrypted[guessed_block_offset..guessed_block_offset + block_len]
        {
            return Some(b);
        }
    }
    None
}

fn guess_block_size<F>(encrypter: F) -> Option<usize>
where
    F: Fn(&[u8]) -> Vec<u8>,
{
    for block_len in 1..=256 {
        let data = vec!['A' as u8; block_len * 3];
        let encrypted = encrypter(&data);
        if encrypted[0..block_len] == encrypted[block_len..block_len * 2]
            && encrypted[0..block_len] == encrypted[block_len * 2..block_len * 3]
        {
            return Some(block_len);
        }
    }
    None
}
