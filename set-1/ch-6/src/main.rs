use my_cryptopals_lib::base64::read_long_base64_bytes_from_stdin;
use my_cryptopals_lib::single_xor;
use std::collections::LinkedList;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_long_base64_bytes_from_stdin()?;
    let key_sizes = guess_key_sizes(&data);
    // (<key size>, [<alphabet for first key char>, <alphabet for second key char >, ...]
    let mut brute_alphabets: Vec<(usize, Vec<Vec<u8>>)> = Vec::new();
    for ks in key_sizes {
        if let Some(alphabet_map) = try_key_size(&data, ks) {
            brute_alphabets.push((ks, alphabet_map));
        }
    }
    let res = brute(&data, brute_alphabets).unwrap();
    print!("Key with score {}: {:?}", res.0, res.1);
    let text_key = std::str::from_utf8(&res.1);
    if let Ok(text_key) = text_key {
        print!(" ({:?})", text_key);
    }
    let decoded = my_cryptopals_lib::xor::decode(&data, &res.1);
    let text = std::str::from_utf8(&decoded).unwrap();
    println!(", result: {:?}", text);
    println!("");

    Ok(())
}

fn brute(data: &[u8], alphabets: Vec<(usize, Vec<Vec<u8>>)>) -> Option<(f64, Vec<u8>)> {
    let mut best_keys: LinkedList<(f64, Vec<u8>)> = LinkedList::new();
    for el in alphabets {
        let (_ks, alphabet) = el;
        let mut key: Vec<u8> = Vec::new();
        let mut state: Vec<u32> = Vec::new();
        let mut is_first = true;
        for _ in 0..alphabet.len() {
            key.push(0);
            state.push(0);
        }
        while permutate(&mut key, &mut state, &mut is_first, &alphabet) {
            if let Some(score) = try_decode(data, &key) {
                try_remember_key(&mut best_keys, &key, score);
            }
        }
    }
    best_keys.pop_back()
}

fn try_decode(data: &[u8], key: &[u8]) -> Option<f64> {
    let decoded = my_cryptopals_lib::xor::decode(data, key);
    let text = std::str::from_utf8(&decoded).ok()?;
    Some(my_cryptopals_lib::lang_stats::compute_english_score(&text))
}

fn try_remember_key(best_keys: &mut LinkedList<(f64, Vec<u8>)>, key: &[u8], score: f64) {
    if best_keys.len() == 10 && best_keys.front().unwrap().0 > score {
        return;
    }
    let mut i = 0;
    for el in best_keys.iter() {
        if score <= el.0 {
            break;
        }
        i += 1;
    }
    let mut rest = best_keys.split_off(i);
    best_keys.push_back((score, key.to_vec()));
    best_keys.append(&mut rest);
    if best_keys.len() > 10 {
        best_keys.pop_front();
    }
}

fn permutate(key: &mut [u8], state: &mut Vec<u32>, is_first: &mut bool, alphabet: &Vec<Vec<u8>>) -> bool {
    if *is_first {
        *is_first = false;
    } else {
        state[0] += 1;
        if state[0] == alphabet[0].len() as u32 {
            let mut j = 1;
            loop {
                state[j - 1] = 0;
                if j == key.len() {
                    return false;
                }
                state[j] += 1;
                if state[j] < alphabet[j].len() as u32 {
                    break;
                }
                j += 1;
            }
        }
    }
    for i in 0..key.len() {
        key[i] = alphabet[i][state[i] as usize];
    }

    true
}

fn try_key_size(data: &[u8], ks: usize) -> Option<Vec<Vec<u8>>> {
    // (<key char index>, <key char>, <score>)
    let mut scores: Vec<(usize, u8, f64)> = Vec::new();
    let mut out: Vec<Vec<u8>> = Vec::new();
    for i in 0..ks {
        out.push(Vec::new());
        let mut data_sample: Vec<u8> = Vec::new();
        for j in (i..data.len()).step_by(ks) {
            data_sample.push(data[j]);
        }
        let brute_res = single_xor::brute(&data_sample, single_xor::ScoringType::EnglishLetters);
        for res in brute_res {
            scores.push((i, res.key, res.score));
        }
    }
    scores.sort_by(|a, b| {
        b.2.partial_cmp(&a.2).unwrap()
    });
    let mut rest_scores: Vec<(usize, u8, f64)> = Vec::new();
    for score in scores {
        let alphabet = &mut out[score.0];
        if alphabet.len() == 0 {
            alphabet.push(score.1);
        } else {
            rest_scores.push(score);
        }
    }
    for i in 0..ks {
        if out[i].len() == 0 {
            return None
        }
    }
    let mut var_count = 1;
    for score in rest_scores {
        let alphabet = &mut out[score.0];
        var_count = var_count / alphabet.len();
        alphabet.push(score.1);
        var_count *= alphabet.len();
        if var_count > 1_000 {
            break;
        }
    }

    Some(out)
}

fn calc_ham_dist(data1: &[u8], data2: &[u8]) -> u32 {
    if data1.len() != data2.len() {
        panic!("Expected data buffers to be same length");
    }
    let mut out = 0;
    for (b1, b2) in data1.iter().zip(data2.iter()) {
        out += (b1 ^ b2).count_ones();
    }

    out
}

fn guess_key_sizes(data: &[u8]) -> Vec<usize> {
    let mut probe_results: Vec<(usize, f32)> = Vec::new();

    for ks in 2..=40.min(data.len() / 4) {
        probe_results.push((ks, probe_key_size(data, ks)));
    }
    probe_results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("Top 5 key sizes: {:?}", &probe_results[0..5]);

    let mut out = Vec::new();
    for res in &probe_results[0..5] {
        out.push(res.0);
    }

    out
}

fn probe_key_size(data: &[u8], ks: usize) -> f32 {
    let mut total_ham = 0;
    let mut probes_done = 0;
    for i in 0..10 {
        let data1_start = i * ks as usize;
        let data2_start = data1_start + ks as usize;
        if (data2_start + ks) > data.len() {
            break;
        }
        let data1 = &data[data1_start .. data1_start + ks];
        let data2 = &data[data2_start .. data2_start + ks];
        total_ham += calc_ham_dist(data1, data2);
        probes_done += 1;
    }
    let norm_ham = total_ham as f32 / (probes_done as f32 * ks as f32);

    norm_ham
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_remember_key_test() {
        let mut best_keys: LinkedList<(f64, Vec<u8>)> = LinkedList::new();
        try_remember_key(&mut best_keys, &[1,2,3], 1.0);
        try_remember_key(&mut best_keys, &[2], 4.0);
        try_remember_key(&mut best_keys, &[3], 0.5);
        try_remember_key(&mut best_keys, &[4], 0.7);
        try_remember_key(&mut best_keys, &[5], 0.8);
        try_remember_key(&mut best_keys, &[6], 0.9);
        try_remember_key(&mut best_keys, &[7], 0.91);
        try_remember_key(&mut best_keys, &[8], 0.92);
        try_remember_key(&mut best_keys, &[9], 0.93);
        try_remember_key(&mut best_keys, &[10], 0.94);
        try_remember_key(&mut best_keys, &[11], 0.95);
        try_remember_key(&mut best_keys, &[12], 5.0);
        try_remember_key(&mut best_keys, &[13], 0.3);
        let mut iter = best_keys.iter();
        assert_eq!(iter.next(), Some(&(0.8, vec![5])));
        assert_eq!(iter.next(), Some(&(0.9, vec![6])));
        assert_eq!(iter.next(), Some(&(0.91, vec![7])));
        assert_eq!(iter.next(), Some(&(0.92, vec![8])));
        assert_eq!(iter.next(), Some(&(0.93, vec![9])));
        assert_eq!(iter.next(), Some(&(0.94, vec![10])));
        assert_eq!(iter.next(), Some(&(0.95, vec![11])));
        assert_eq!(iter.next(), Some(&(1.0, vec![1, 2, 3])));
        assert_eq!(iter.next(), Some(&(4.0, vec![2])));
        assert_eq!(iter.next(), Some(&(5.0, vec![12])));
    }
}
