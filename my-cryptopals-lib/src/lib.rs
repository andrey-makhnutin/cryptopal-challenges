use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub mod lang_stats;
pub mod syllables;
mod eng_syllables;

pub fn read_hex_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read from stdin");
    if buf.ends_with('\n') {
        buf.truncate(buf.len() - 1);
    }
    parse_hex_string(&buf)
}

pub fn read_long_hex_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut buf = String::new();
    println!("Enter hex strings followed by empty line");
    loop {
        let mut part_buf = String::new();
        io::stdin().read_line(&mut part_buf).expect("Failed to read from stdin");
        if part_buf.ends_with('\n') {
            part_buf.truncate(part_buf.len() - 1);
        }
        let part_buf = part_buf.trim();
        if part_buf.len() == 0 {
            break;
        }
        buf.push_str(part_buf);
    }
    parse_hex_string(&buf)
}

pub fn read_long_string_from_stdin() -> Vec<u8> {
    let mut buf = String::new();
    println!("Enter hex strings followed by empty line");
    loop {
        let mut part_buf = String::new();
        io::stdin().read_line(&mut part_buf).expect("Failed to read from stdin");
        let trimmed_part_buf = part_buf.trim();
        if trimmed_part_buf.len() == 0 {
            break;
        }
        buf.push_str(&part_buf);
    };
    Vec::from(buf.as_bytes())
}

pub fn parse_hex_string(text: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(text.len() / 2);
    // most significant nibble
    let mut msn = true;
    let mut next_num = 0u8;
    for (i, c) in text.char_indices() {
        let nibble = c.to_digit(16);
        if nibble == None {
            let rest_of_str = &text[i..];
            let mut graphemes = rest_of_str.graphemes(true);
            let mut next_grapheme = graphemes.next().unwrap();
            if i > 0 {
                let prev_graphemes = &text[i - 1..];
                let mut prev_graphemes = prev_graphemes.graphemes(true);
                let prev_grapheme = prev_graphemes.next().unwrap();
                if prev_grapheme.len() > next_grapheme.len() {
                    next_grapheme = prev_grapheme;
                }
            }
            return Err(format!("Expected only hex digits in a string, got '{}'", next_grapheme));
        }
        let nibble = nibble.unwrap() as u8;

        if msn {
            next_num = nibble << 4;
            msn = false;
        } else {
            next_num |= nibble;
            out.push(next_num);
            msn = true;
        }
    }
    if !msn {
        return Err("hex string has odd number of digits".into());
    }
    Ok(out)
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
    let mut out = String::new();
    let mut buffer = 0u8;
    let mut sextets_processed = 0;
    for byte in bytes {
        match sextets_processed {
            0 => {
                let next_sextet = byte >> 2;
                out.push(calc_sextet(next_sextet));
                buffer = (byte & 0b11) << 4;
                sextets_processed = 1;
            },
            1 => {
                let next_sextet = buffer | (byte >> 4);
                out.push(calc_sextet(next_sextet));
                buffer = (byte & 0b1111) << 2;
                sextets_processed = 2;
            },
            _ => {
                let next_sextet = buffer | (byte >> 6);
                out.push(calc_sextet(next_sextet));
                let next_sextet = byte & 0b111111;
                out.push(calc_sextet(next_sextet));
                sextets_processed = 0;
            }
        }
    }
    match sextets_processed {
        1 => {
            out.push(calc_sextet(buffer));
            out.push_str("==");
        },
        2 => {
            out.push(calc_sextet(buffer));
            out.push_str("=");
        },
        _ => ()
    }
    out
}

fn calc_sextet(sextet: u8) -> char {
    match sextet {
        0 ..= 25 => {
            return (b'A' + sextet) as char;
        },
        26 ..= 51 => {
            return (b'a' + sextet - 26) as char;
        },
        52 ..= 61 => {
            return (b'0' + sextet - 52) as char;
        },
        62 => return '+',
        63 => return '-',
        _ => panic!("Invalid base64 sextet {}", sextet)
    }
}

pub fn print_hex_bytes(bytes: &[u8]) {
    for b in bytes {
        print!(
            "{}{}",
            char::from_digit((b >> 4) as u32, 16).unwrap(),
            char::from_digit((b & 0b1111) as u32, 16).unwrap()
        );
    }
    println!()
}


pub fn try_print_utf8(bytes: &[u8]) {
    let res = std::str::from_utf8(bytes);
    if let Ok(str) = res {
        println!("  - can also be decoded as utf8: \"{}\"", str);
    }
}
