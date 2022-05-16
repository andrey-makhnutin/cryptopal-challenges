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
