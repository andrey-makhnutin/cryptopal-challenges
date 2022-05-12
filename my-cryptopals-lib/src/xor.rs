pub fn decode(data: &[u8], key: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = vec![0; data.len()];

    for i in 0..data.len() {
        out[i] = data[i] ^ key[i % key.len()];
    }

    out
}
