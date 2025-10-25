use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut word_chars: Vec<char> = word.to_lowercase().chars().collect::<Vec<char>>();
    word_chars.sort_unstable();

    let mut found_anagrams = HashSet::new();
    for &anagram in possible_anagrams {
        let mut anagram_chars: Vec<char> = anagram.to_lowercase().chars().collect::<Vec<char>>();
        anagram_chars.sort_unstable();

        if anagram.to_lowercase() != word.to_lowercase() && anagram_chars == word_chars {
            found_anagrams.insert(anagram);
        }
    }

    found_anagrams
}
