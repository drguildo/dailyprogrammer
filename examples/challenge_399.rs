use std::collections::{HashMap, HashSet};

fn main() {
    let challenge_data = std::fs::read_to_string(r"data\challenge_399_enable1.txt")
        .expect("Failed to open words list");
    let words = challenge_data.lines();

    let mut word_to_lettersum: HashMap<&str, u32> = HashMap::new();
    let mut letter_length_groupings: HashMap<usize, Vec<(&str, u32)>> = HashMap::new();
    for word in words {
        let lettersum = lettersum(word);
        word_to_lettersum.insert(word, lettersum);

        let length = word.len();
        if let Some(grouping) = letter_length_groupings.get_mut(&length) {
            grouping.push((word, lettersum));
        } else {
            let new_grouping = vec![(word, lettersum)];
            letter_length_groupings.insert(length, new_grouping);
        }
    }

    optional_1(&word_to_lettersum);
    optional_2(&word_to_lettersum);
    optional_3(&word_to_lettersum);
    optional_4(&letter_length_groupings);
}

fn lettersum(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - 'a' as u32 + 1).sum()
}

// `microspectrophotometries` is the only word with a letter sum of 317. Find
// the only word with a letter sum of 319.
fn optional_1(word_to_lettersum: &HashMap<&str, u32>) {
    if let Some((key, _)) = word_to_lettersum.iter().find(|&(_, &v)| v == 319) {
        println!("The word with the sum 319 is \"{}\"", key);
    } else {
        println!("No word found with a sum of 319");
    }
}

// How many words have an odd letter sum?
fn optional_2(word_to_lettersum: &HashMap<&str, u32>) {
    let num_odd_lettersums =
        word_to_lettersum
            .iter()
            .fold(0, |acc, kv| if kv.1 % 2 == 1 { acc + 1 } else { acc });
    println!(
        "The number of words with odd lettersums is {}",
        num_odd_lettersums
    );
}

// There are 1921 words with a letter sum of 100, making it the second most
// common letter sum. What letter sum is most common, and how many words have
// it?
fn optional_3(word_to_lettersum: &HashMap<&str, u32>) {
    let mut lettersum_frequencies: HashMap<u32, u32> = HashMap::new();
    for (_, v) in word_to_lettersum {
        if let Some(lettersum) = lettersum_frequencies.get_mut(v) {
            *lettersum += 1;
        } else {
            lettersum_frequencies.insert(*v, 1);
        }
    }

    let mut sorted_frequencies = lettersum_frequencies
        .values()
        .cloned()
        .collect::<HashSet<u32>>()
        .into_iter()
        .collect::<Vec<u32>>();
    sorted_frequencies.sort_unstable();

    if let Some(most_frequent) = sorted_frequencies.last() {
        if let Some((lettersum, frequency)) = lettersum_frequencies
            .iter()
            .find(|&(_, &v)| v == *most_frequent)
        {
            println!(
                "{} words have the most frequently encountered lettersum of {}",
                frequency, lettersum
            );
        }
    }
}

// `zyzzyva` and `biodegradabilities` have the same letter sum as each other
// (151), and their lengths differ by 11 letters. Find the other pair of words
// with the same letter sum whose lengths differ by 11 letters.
fn optional_4(letter_length_groupings: &HashMap<usize, Vec<(&str, u32)>>) {
    let mut letter_lengths: Vec<&usize> = letter_length_groupings.keys().collect();
    letter_lengths.sort_by(|a, b| b.cmp(a));

    for length in letter_lengths {
        let longer = letter_length_groupings.get(length).unwrap();
        if let Some(shorter) = letter_length_groupings.get(&(length - 11)) {
            for long in longer {
                if let Some(found) = shorter.iter().find(|short| long.1 == short.1) {
                    println!(
                        "\"{}\" has the same lettersum as \"{}\" but differs in length by 11",
                        long.0, found.0
                    );
                    return;
                }
            }
        }
    }
}

// `cytotoxicity` and `unreservedness` have the same letter sum as each other
// (188), and they have no letters in common. Find a pair of words that have no
// letters in common, and that have the same letter sum, which is larger than
// 188. (There are two such pairs, and one word appears in both pairs.)
fn optional_5(word_to_lettersum: &HashMap<&str, u32>) {
    todo!()
}

// The list of word `{ geographically, eavesdropper, woodworker, oxymorons }`
// contains 4 words. Each word in the list has both a different number of
// letters, and a different letter sum. The list is sorted both in descending
// order of word length, and ascending order of letter sum. What's the longest
// such list you can find?
fn optional_6(word_to_lettersum: &HashMap<&str, u32>) {
    todo!()
}
