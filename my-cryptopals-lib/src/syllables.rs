pub fn split_into_syllables(text: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();

    let mut end_of_syllable = false;
    let mut new_syllable = String::new();
    let mut tail = 0;
    let mut consonants_in_a_row = 0;
    for (i, c) in text.char_indices() {
        if is_word_boundary(c) {
            if new_syllable.len() > 0 {
                out.push(new_syllable);
                new_syllable = String::new();
            }
            end_of_syllable = false;
            tail = 0;
            continue;
        }
        new_syllable.push(c);
        if tail > 0 {
            tail -= 1;
            if tail == 0 {
                end_of_syllable = true;
            }
        } else if is_vowel(c) {
            consonants_in_a_row = 0;
            let rest_of_text = &text[i + 1..];
            if next_is_single_consonant(rest_of_text) {
                end_of_syllable = true;
            } else if next_is_consonant_and_vowels(rest_of_text) {
                tail = 1;
            }
        } else {
            consonants_in_a_row += 1;
            if consonants_in_a_row > 5 {
                end_of_syllable = true;
            }
        }
        if end_of_syllable {
            out.push(new_syllable);
            new_syllable = String::new();
            end_of_syllable = false;
        }
    }
    if new_syllable.len() > 0 {
        out.push(new_syllable);
    }

    out
}

fn is_word_boundary(c: char) -> bool {
    if c.is_whitespace() {
        return true;
    }
    match c {
        ','|'.'|':'|';'|'|'|'/'|'\\'|'\''|'"'|'`'|'~'|'@'
            |'!'|'#'|'$'|'%'|'^'|'&'|'*'|'('|')'|'-'|'_'|'+'
            |'['|']'|'{'|'}'|'<'|'>'|'?'|'—'|'’'|'…'|'“'|'‘'|'”' => true,
        '0'..='9' => true,
        _ => false,
    }
}

fn is_vowel(c: char) -> bool {
    let uc_str = c.to_uppercase().to_string();
    if uc_str.len() > 1 {
        return false;
    }
    let uc = uc_str.chars().next().unwrap();
    match uc {
        'A' | 'E' | 'I' | 'O' | 'U' | 'Y' => true,
        _ => false
    }
}

fn next_is_single_consonant(text: &str) -> bool {
    let mut it = text.chars();
    let next = it.next();
    if next == None {
        return false;
    }
    let next = next.unwrap();
    if !next.is_alphabetic() || is_vowel(next) {
        return false;
    }

    let after_next = it.next();
    if after_next == None {
        return false;
    }
    let after_next = after_next.unwrap();
    if after_next.is_alphabetic() && is_vowel(after_next) {
        return true;
    }
    return false;
}

fn next_is_consonant_and_vowels(text: &str) -> bool {
    let mut it = text.chars();
    let next = it.next();
    if next == None {
        return false;
    }
    let next = next.unwrap();
    if !next.is_alphabetic() || is_vowel(next) {
        return false;
    }

    let mut vowel_found = false;
    let mut consonant_found = false;
    for c in it {
        if !c.is_alphabetic() {
            return false;
        }
        if is_vowel(c) {
            vowel_found = true;
        } else {
            consonant_found = true;
        }
        if consonant_found && vowel_found {
            return true;
        }
    }
    return false;
}
