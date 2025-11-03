use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    HashSet::from_iter(
        possible_anagrams
            .iter()
            .copied()
            .filter(|item| is_anagram(word, item)),
    )
}

pub fn is_anagram(word: &str, possible_anagram: &str) -> bool {
    let lowercased_word = word.to_lowercase();
    let lowercased_possible_anagram = possible_anagram.to_lowercase();
    if lowercased_word == lowercased_possible_anagram {
        return false;
    }
    let mut sorted_word: Vec<char> = lowercased_word.chars().collect();
    sorted_word.sort_unstable();

    let mut sorted_possible_anagram: Vec<char> = lowercased_possible_anagram.chars().collect();
    sorted_possible_anagram.sort_unstable();

    String::from_iter(sorted_word) == String::from_iter(sorted_possible_anagram)
}
