use my_cryptopals_lib::lang_stats::compute_english_score;
use my_cryptopals_lib::read_hex_bytes_from_stdin;

fn main() -> Result<(), String> {
    println!("Enter XOR-enctypted data in hex");
    let data = read_hex_bytes_from_stdin()?;

    let mut best_score: Option<f64> = None;
    let mut best_key: Option<u8> = None;
    let mut best_string: Option<String> = None;
    for k in 0..=0xff {
        let text = try_xor_to_utf8(&data, k);
        if text == None {
            continue;
        }
        let text = text.unwrap();
        let score = compute_english_score(&text);
        if best_score == None || best_score.unwrap() > score {
            best_score = Some(score);
            best_key = Some(k);
            best_string = Some(text);
        }
    }
    if best_score == None {
        println!("Failed to decode string");
    } else {
        println!(
            "Best key found was {:x} with score {},\ndecoded string: \"{}\"",
            best_key.unwrap(),
            best_score.unwrap(),
            best_string.unwrap()
        );
    }

    Ok(())
}

fn try_xor_to_utf8(data: &[u8], k: u8) -> Option<String> {
    let mut decoded = data.to_vec();
    for byte in decoded.iter_mut() {
        *byte ^= k;
    }
    let res = std::str::from_utf8(&decoded);
    if let Ok(str_) = res {
        return Some(str_.to_owned());
    }
    None
}
