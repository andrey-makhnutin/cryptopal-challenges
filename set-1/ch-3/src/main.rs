use my_cryptopals_lib::read_hex_bytes_from_stdin;
use my_cryptopals_lib::single_xor;

fn main() -> Result<(), String> {
    println!("Enter XOR-enctypted data in hex");
    let data = read_hex_bytes_from_stdin()?;

    let res = single_xor::try_decode(&data, single_xor::ScoringType::EnglishSyllables);
    if let Some(res) = res {
        let decoded = single_xor::decode(&data, res.key);
        let text = std::str::from_utf8(&decoded).unwrap();
        println!(
            "Best key found was 0x{:x} with score {},\ndecoded string: \"{}\"",
            res.key,
            res.score,
            text
        );
    } else {
        println!("Failed to decode string");
    }

    Ok(())
}
