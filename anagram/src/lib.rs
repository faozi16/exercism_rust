use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {

    let word_lower = word.to_lowercase();
    let word_sorted = sort_string(&word_lower);

    possible_anagrams
        .iter()
        .filter(|&&candidate| {
                let candidate_lower = candidate.to_lowercase();
                candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && sort_string(&candidate_lower) == word_sorted
        })
        .copied()
        .collect()
}

fn sort_string(word_lower: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word_lower.chars().collect();
    word_chars.sort_unstable();
    word_chars
}
