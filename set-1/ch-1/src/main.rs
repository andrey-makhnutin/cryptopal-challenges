use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    bytes_to_base64
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
    println!("{}", bytes_to_base64(&bytes));

    return Ok(());
}
