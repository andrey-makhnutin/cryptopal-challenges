use my_cryptopals_lib;

fn main() -> Result<(), String> {
    let data = my_cryptopals_lib::read_long_bytes_from_stdin()?;
    let text = std::str::from_utf8(&data)
        .or_else(|_| Err(format!("Data can't be parsed as utf8 string")))?;

    let syllables = my_cryptopals_lib::syllables::split_into_syllables(text);
    println!("Found {} syllables: {:?}", syllables.len(), syllables);
    println!("Syllable score: {}", my_cryptopals_lib::lang_stats::compute_english_syllables_score(text));
    println!("Letter score: {}", my_cryptopals_lib::lang_stats::compute_english_letters_score(text));
    println!("Total score: {}", my_cryptopals_lib::lang_stats::compute_english_score(text));

    Ok(())
}
