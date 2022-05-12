use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    print_hex_bytes,
    single_xor,
};

fn main() -> Result<(), String> {
    loop {
        let data = read_hex_bytes_from_stdin()?;
        if data.len() == 0 {
            break;
        }
        let res = single_xor::try_decode(&data, single_xor::ScoringType::EnglishSyllables);
        if let Some(res) = res {
            if res.score > 0.1 {
                let decoded = single_xor::decode(&data, res.key);
                let text = std::str::from_utf8(&decoded).unwrap();
                print!("Decoded string: ");
                print_hex_bytes(&data);
                println!("'{:?}', score {}", text, res.score);
            }
        }
    }
    Ok(())
}
