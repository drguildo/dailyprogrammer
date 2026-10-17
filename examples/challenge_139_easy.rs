use std::collections::HashMap;

fn main() {
    let mut s = String::from("The quick brown fox jumps over the lazy dog");

    println!("s: {}", is_pangram(&s));

    println!(
        "The quick brown fox jumps over the lazy dog: {}",
        is_pangram("The quick brown fox jumps over the lazy dog")
    );

    s = String::from("Vasa inania multum strepunt");
    println!("{}: {}", s, is_pangram(&s));
}

/// Checks if a given string is a pangram (contains every letter of the alphabet
/// at least once).
fn is_pangram(s: &str) -> bool {
    let counts = character_counts(s);

    for ch in 'a'..='z' {
        if !counts.contains_key(&ch) {
            return false;
        }
    }

    true
}

/// Checks the number of occurrences of each character in a given string.
fn character_counts(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();

    for ch in s.to_lowercase().chars() {
        if let Some(count) = counts.get_mut(&ch) {
            *count += 1;
        } else {
            counts.insert(ch, 1);
        }
    }

    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let counts = character_counts("The quick brown fox jumps over the lazy dog");
        assert_eq!(counts.get(&'a'), Some(&1));
        assert_eq!(counts.get(&'b'), Some(&1));
        assert_eq!(counts.get(&'c'), Some(&1));
        assert_eq!(counts.get(&'d'), Some(&1));
        assert_eq!(counts.get(&'e'), Some(&3));
        assert_eq!(counts.get(&'f'), Some(&1));
        assert_eq!(counts.get(&'g'), Some(&1));
        assert_eq!(counts.get(&'h'), Some(&2));
        assert_eq!(counts.get(&'i'), Some(&1));
        assert_eq!(counts.get(&'j'), Some(&1));
        assert_eq!(counts.get(&'k'), Some(&1));
        assert_eq!(counts.get(&'l'), Some(&1));
        assert_eq!(counts.get(&'m'), Some(&1));
        assert_eq!(counts.get(&'n'), Some(&1));
        assert_eq!(counts.get(&'o'), Some(&4));
        assert_eq!(counts.get(&'p'), Some(&1));
        assert_eq!(counts.get(&'q'), Some(&1));
        assert_eq!(counts.get(&'r'), Some(&2));
        assert_eq!(counts.get(&'s'), Some(&1));
        assert_eq!(counts.get(&'t'), Some(&2));
        assert_eq!(counts.get(&'u'), Some(&2));
        assert_eq!(counts.get(&'v'), Some(&1));
        assert_eq!(counts.get(&'w'), Some(&1));
        assert_eq!(counts.get(&'x'), Some(&1));
        assert_eq!(counts.get(&'y'), Some(&1));
        assert_eq!(counts.get(&'z'), Some(&1));
    }

    #[test]
    fn test_is_pangram() {
        assert_eq!(
            is_pangram("The quick brown fox jumps over the lazy dog"),
            true
        );
    }

    #[test]
    fn test_not_pangram() {
        assert_eq!(is_pangram("Vasa inania multum strepunt"), false);
    }
}
