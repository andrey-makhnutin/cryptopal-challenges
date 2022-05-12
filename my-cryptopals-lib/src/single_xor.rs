use std::cmp::Ordering;

pub enum ScoringType {
    EnglishLetters,
    EnglishSyllables,
}

#[derive(Clone, Copy, Debug)]
pub struct DecodeResult {
    pub key: u8,
    pub score: f64,
}

pub fn try_decode(data: &[u8], scoring_type: ScoringType) -> Option<DecodeResult> {
    let results = brute(data, scoring_type);
    if results.is_empty() {
        None
    } else {
        Some(results[0])
    }
}

pub fn decode(data: &[u8], key: u8) -> Vec<u8> {
    let mut out = data.to_vec();
    for i in 0..out.len() {
        out[i] ^= key;
    }
    out
}

pub fn brute(data: &[u8], scoring_type: ScoringType) -> Vec<DecodeResult> {
    let mut out = Vec::new();

    let mut test_data = data.to_vec();
    for key in 0..=0xff {
        for i in 0..data.len() {
            test_data[i] = data[i] ^ key;
        }
        let utf8_str = std::str::from_utf8(&test_data);
        if utf8_str.is_err() {
            continue;
        }
        let utf8_str = utf8_str.unwrap();
        let score = match scoring_type {
            ScoringType::EnglishLetters => {
                crate::lang_stats::compute_english_letters_score(utf8_str)
            }
            ScoringType::EnglishSyllables => crate::lang_stats::compute_english_score(utf8_str),
        };
        out.push(DecodeResult { key, score });
    }

    out.sort();
    out.reverse();
    out
}

impl Ord for DecodeResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for DecodeResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl PartialEq for DecodeResult {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Eq for DecodeResult {}
