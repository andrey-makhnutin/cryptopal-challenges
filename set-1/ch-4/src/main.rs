use my_cryptopals_lib::{
    read_hex_bytes_from_stdin,
    print_hex_bytes,
};
use single_xor_decoder_s1_ch3::try_decode_single_xor;

fn main() -> Result<(), String> {
    loop {
        let data = read_hex_bytes_from_stdin()?;
        if data.len() == 0 {
            break;
        }
        let res = try_decode_single_xor(&data);
        if let Some(res) = res {
            if res.score > 0.15 {
                print!("Decoded string: ");
                print_hex_bytes(&data);
                println!("'{:?}', score {}", res.text, res.score);
            }
        }
    }
    Ok(())
}
