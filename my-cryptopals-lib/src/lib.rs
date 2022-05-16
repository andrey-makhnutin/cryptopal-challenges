use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub mod lang_stats;
pub mod syllables;
mod eng_syllables;
pub mod base64;
pub mod single_xor;
pub mod xor;
pub mod aes;
pub mod block_cipher;

pub fn read_long_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut buf = String::new();
    println!("Enter bytes followed by empty line. Optional prefixes: 'str', 'esc', 'b64', 'hex'");
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
    if buf.len() < 3 {
        return Ok(buf.as_bytes().to_vec());
    }
    let prefix = &buf[0..3];
    let rest = &buf[3..];
    match prefix {
        "str" => Ok(rest.as_bytes().to_vec()),
        "esc" => unescape_string(rest),
        "b64" => base64::parse_base64_string(rest),
        "hex" => parse_hex_string(rest),
        _ => Ok(buf.as_bytes().to_vec()),
    }
}

fn unescape_string(s: &str) -> Result<Vec<u8>, String> {
    let mut out = String::new();
    out.reserve(s.len());
    let mut found_escape = false;
    let mut in_unicode_escape = false;
    let mut unicode_point = String::new();
    let s = if s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    };
    for c in s.chars() {
        if found_escape {
            match c {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                '\'' => out.push('\''),
                '"' => out.push('"'),
                '\\' => out.push('\\'),
                'u' => {
                    in_unicode_escape = true;
                    unicode_point = String::new();
                },
                _ => return Err(format!("Unknown escape symbol {}", c)),
            }
            found_escape = false;
            continue;
        }
        if in_unicode_escape {
            match c {
                '{' => (),
                '}' => {
                    in_unicode_escape = false;
                    let uc_point = u32::from_str_radix(&unicode_point, 16);
                    if uc_point.is_err() {
                        return Err(format!("Failed to parse unicode code point {}", unicode_point));
                    }
                    let uc_point = uc_point.unwrap();
                    let uc_char = char::from_u32(uc_point);
                    if uc_char == None {
                        return Err(format!("Invalid unicode code point {}", unicode_point));
                    }
                    out.push(uc_char.unwrap());
                },
                _ => unicode_point.push(c),
            }
            continue;
        }
        match c {
            '\\' => found_escape = true,
            _ => out.push(c),
        }
    }
    Ok(out.as_bytes().to_vec())
}

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
    println!("Enter strings followed by empty line");
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
        63 => return '/',
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

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(char::from_digit((b >> 4) as u32, 16).unwrap());
        out.push(char::from_digit((b & 0b1111) as u32, 16).unwrap());
    }

    out
}

pub fn try_print_utf8(bytes: &[u8]) {
    let res = std::str::from_utf8(bytes);
    if let Ok(str) = res {
        println!("  - can also be decoded as utf8: {:?}", str);
    }
}

fn escape_string_with_newlines(s: &str) -> String {
    let debug_str = s.to_string();
    let mut out = String::with_capacity(debug_str.len() + 6);
    out.push_str("esc\"\n");
    let mut found_escape = false;
    for c in s.escape_debug() {
        if found_escape {
            found_escape = false;
            if c == 'n' {
                out.push('\n');
                continue;
            } else if c == '\'' {
                out.push('\'');
                continue;
            } else if c == '"' {
                out.push('"');
                continue;
            } else {
                out.push('\\');
            }
        } else {
            if c == '\\' {
                found_escape = true;
                continue;
            }
        }
        out.push(c);
    }
    out.push('"');
    out
}

pub fn bytes_to_str_or_hex(bytes: &[u8]) -> String {
    match std::str::from_utf8(bytes) {
        Ok(utf8_str) => escape_string_with_newlines(utf8_str),
        Err(_) => format!("hex{}", bytes_to_hex(bytes)),
    }
}
