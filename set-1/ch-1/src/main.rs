use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    bytes_to_base64,
    try_print_utf8
};

fn main() {
    match main_() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn main_() -> Result<(), String> {
    let bytes = read_hex_bytes_from_stdin()?;
    try_print_utf8(&bytes);
    println!("{}", bytes_to_base64(&bytes));

    return Ok(());
}
