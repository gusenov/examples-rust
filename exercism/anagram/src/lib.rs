use std::collections::HashSet;

const BASE: usize = 'a' as usize;

pub fn ascii_anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_ascii_lowercase();

    let mut alphabet = [0; 26];
    for ch in lowercase_word.chars() {
        let i = ch as usize - BASE;
        alphabet[i] += 1;
    }

    println!("{:?}", alphabet);

    let mut result: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_ascii_lowercase();
        if std::cmp::Ordering::Equal == lowercase_word.cmp(&lowercase_possible_anagram) {
            continue
        }
        let mut tmp_alphabet = alphabet;
        for ch in lowercase_possible_anagram.chars() {
            let i = ch as usize - BASE;
            tmp_alphabet[i] -= 1;
        }

        println!("{:?}", tmp_alphabet);

        if tmp_alphabet.iter().all(|x| 0 == *x) {
            result.insert(possible_anagram);
        }
    }

    result
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let lowercase_word = word.to_lowercase();

    let mut letters: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
    for ch in lowercase_word.chars() {
        match letters.get(&ch) {
            Some(count) => { letters.insert(ch, count + 1); },
            None => { letters.insert(ch, 1); },
        }
    }
    let letters = letters;
    println!("{:?}", letters);

    let mut result: HashSet<&'a str> = HashSet::new();

    for possible_anagram in possible_anagrams {
        let lowercase_possible_anagram = possible_anagram.to_lowercase();
        if std::cmp::Ordering::Equal == lowercase_word.cmp(&lowercase_possible_anagram) {
            continue
        }

        let mut tmp_letters = letters.clone();
        for ch in lowercase_possible_anagram.chars() {
            match tmp_letters.get(&ch) {
                Some(count) => { tmp_letters.insert(ch, count - 1); },
                None => { tmp_letters.insert(ch, 1); },
            }
        }

        if tmp_letters.iter().all(|(_, count)| 0 == *count) {
            result.insert(possible_anagram);
        }
    }

    result
}
