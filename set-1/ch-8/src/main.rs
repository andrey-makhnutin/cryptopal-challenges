use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    print_hex_bytes,
};
use std::collections::HashSet;

fn main() -> Result<(), String> {
    loop {
        let data = read_hex_bytes_from_stdin()?;
        if data.len() == 0 {
            break;
        }
        check_for_ecb(&data);
    }
    Ok(())
}

fn check_for_ecb(data: &[u8]) {
    if data.len() % 16 != 0 {
        println!("Found data with len not divisible by 16");
        print_hex_bytes(data);
        return;
    }
    let mut blocks: HashSet<&[u8]> = HashSet::new();
    let mut repeating_blocks: HashSet<&[u8]> = HashSet::new();
    for i in (0..data.len()).step_by(16) {
        let block = &data[i..i+16];
        if blocks.contains(block) {
            repeating_blocks.insert(block);
        }
        blocks.insert(block);
    }
    if repeating_blocks.len() > 0 {
        print!("Found data likely encoded by ECB cipher: ");
        print_hex_bytes(data);
        println!("Repeating blocks:");
        for block in repeating_blocks {
            print_hex_bytes(block);
        }
        println!("")
    }
}
