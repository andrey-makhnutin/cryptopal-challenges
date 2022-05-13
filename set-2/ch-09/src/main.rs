use std::io;
use std::io::Write;
use my_cryptopals_lib::{
    read_long_bytes_from_stdin,
    try_print_utf8,
};

fn main() -> Result<(), String> {
    let mut data = read_long_bytes_from_stdin()?;
    let len = read_int("Enter block length: ");
    pad(&mut data, len);
    try_print_utf8(&data);
    Ok(())
}

fn read_int(prompt: &str) -> usize {
    print!("{}", prompt);
    let stdin = io::stdin();
    io::stdout().flush().unwrap();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let line = line.trim();
    usize::from_str_radix(&line, 10).unwrap()
}

fn pad(data: &mut Vec<u8>, block_len: usize) {
    if block_len > 256 {
        panic!("Block size can't be bigger than 256 bytes");
    }
    let pad_len = block_len - data.len() % block_len;
    let pad_byte = pad_len as u8;
    let mut pad_vec = vec![pad_byte; pad_len];
    data.append(&mut pad_vec);
}
