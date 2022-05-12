use std::io;
use unicode_segmentation::UnicodeSegmentation;

pub fn read_long_base64_bytes_from_stdin() -> Result<Vec<u8>, String> {
    let mut buf = String::new();
    println!("Enter base64 strings followed by empty line");
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
    parse_base64_string(&buf)
}

pub fn parse_base64_string(text: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();

    let mut buffer = 0u8;
    let mut sextets_processed = 0;
    let mut last_index = 0;
    for (i, c) in text.char_indices() {
        last_index = i;
        if c == '=' {
            break;
        }
        let sextet = parse_sextet(c);
        if sextet == None {
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
            return Err(format!("Expected only base64 symbols in a string, got '{}'", next_grapheme));
        }
        let sextet = sextet.unwrap();
        match sextets_processed {
            0 => buffer = sextet << 2,
            1 => {
                buffer |= sextet >> 4;
                out.push(buffer);
                buffer = sextet << 4;
            },
            2 => {
                buffer |= sextet >> 2;
                out.push(buffer);
                buffer = sextet << 6;
            },
            _ => {
                buffer |= sextet;
                out.push(buffer);
            },
        };
        sextets_processed = (sextets_processed + 1) % 4;
    }
    // if sextets_processed == 0 && text.len() >= 4 {
    //     out.push(buffer);
    // }
    if text.len() - last_index > 2 {
        return Err(format!("Found some junk at the end: {}", &text[last_index + 1..]));
    } else if text.len() - last_index == 2 && &text[last_index + 1..] != "=" {
        return Err(format!("Expected '=' as padding at the end of a string, got '{}'", &text[last_index + 1 ..]))
    }

    Ok(out)
}

fn parse_sextet(c: char) -> Option<u8> {
    let mut utf16_bytes = [0u16, 2];
    c.encode_utf16(&mut utf16_bytes);
    let lsb = utf16_bytes[0] as u8;
    match c {
        'A' ..= 'Z' => Some(lsb - b'A'),
        'a' ..= 'z' => Some(lsb - b'a' + 26),
        '0' ..= '9' => Some(lsb - b'0' + 52),
        '+' => Some(62),
        '/' => Some(63),
        _ => None
    }
}
