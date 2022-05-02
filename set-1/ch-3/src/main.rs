
use my_cryptopals_lib::read_hex_bytes_from_stdin;

mod lib;

fn main() -> Result<(), String> {
    println!("Enter XOR-enctypted data in hex");
    let data = read_hex_bytes_from_stdin()?;

    let res = lib::try_decode_single_xor(&data);
    if let Some(res) = res {
        println!(
            "Best key found was 0x{:x} with score {},\ndecoded string: \"{}\"",
            res.key,
            res.score,
            res.text
        );
    } else {
        println!("Failed to decode string");
    }

    Ok(())
}
