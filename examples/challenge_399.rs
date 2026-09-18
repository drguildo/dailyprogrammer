use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy)]
struct Word<'a> {
    text: &'a str,
    length: usize,
    letter_sum: u32,
    letter_mask: u32,
}

fn main() {
    let challenge_data = std::fs::read_to_string("data/challenge_399_enable1.txt")
        .expect("Failed to open words list");
    let words = challenge_data.lines();

    let words: Vec<Word> = words
        .map(|text| Word {
            text,
            length: text.len(),
            letter_sum: lettersum(text),
            letter_mask: letter_mask(text),
        })
        .collect();

    optional_1(&words);
    optional_2(&words);
    optional_3(&words);
    optional_4(&words);
    optional_5(&words);
    optional_6(&words);
}

// Returns the sum of the positions of the letters in the alphabet for the given
// word. 'a' is 1, 'b' is 2, etc.
fn lettersum(s: &str) -> u32 {
    s.chars().map(|c| c as u32 - 'a' as u32 + 1).sum()
}

// Returns a bitmask representing the letters present in the word. Each bit
// corresponds to a letter from 'a' to 'z'.
fn letter_mask(s: &str) -> u32 {
    s.chars()
        .fold(0, |mask, c| mask | (1 << (c as u32 - 'a' as u32)))
}

// `microspectrophotometries` is the only word with a letter sum of 317. Find
// the only word with a letter sum of 319.
fn optional_1(words: &[Word]) {
    if let Some(word) = words.iter().find(|word| word.letter_sum == 319) {
        println!("The word with the sum 319 is \"{}\"", word.text);
    } else {
        println!("No word found with a sum of 319");
    }
}

// How many words have an odd letter sum?
fn optional_2(words: &[Word]) {
    let num_odd_lettersums = words.iter().filter(|word| word.letter_sum % 2 == 1).count();
    println!(
        "The number of words with odd lettersums is {}",
        num_odd_lettersums
    );
}

// There are 1921 words with a letter sum of 100, making it the second most
// common letter sum. What letter sum is most common, and how many words have
// it?
fn optional_3(words: &[Word]) {
    let mut lettersum_frequencies: HashMap<u32, u32> = HashMap::new();
    for word in words {
        if let Some(lettersum) = lettersum_frequencies.get_mut(&word.letter_sum) {
            *lettersum += 1;
        } else {
            lettersum_frequencies.insert(word.letter_sum, 1);
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
fn optional_4(words: &[Word]) {
    let mut letter_length_groupings: HashMap<usize, Vec<&Word>> = HashMap::new();
    for word in words {
        letter_length_groupings
            .entry(word.length)
            .or_default()
            .push(word);
    }

    let mut letter_lengths: Vec<&usize> = letter_length_groupings.keys().collect();
    letter_lengths.sort_by(|a, b| b.cmp(a));

    for length in letter_lengths {
        let longer = letter_length_groupings.get(length).unwrap();
        if let Some(shorter) = length
            .checked_sub(11)
            .and_then(|length| letter_length_groupings.get(&length))
        {
            for long in longer {
                if let Some(found) = shorter
                    .iter()
                    .find(|short| long.letter_sum == short.letter_sum)
                {
                    println!(
                        "\"{}\" has the same lettersum as \"{}\" but differs in length by 11",
                        long.text, found.text
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
fn optional_5(words: &[Word]) {
    let mut lettersum_groupings: HashMap<u32, Vec<&Word>> = HashMap::new();
    for word in words {
        lettersum_groupings
            .entry(word.letter_sum)
            .or_default()
            .push(word);
    }

    let mut matches = Vec::new();
    for (lettersum, words) in lettersum_groupings {
        if lettersum <= 188 {
            continue;
        }

        for (index, first) in words.iter().enumerate() {
            for second in words.iter().skip(index + 1) {
                if first.letter_mask & second.letter_mask == 0 {
                    matches.push((first.text, second.text, lettersum));
                }
            }
        }
    }

    matches.sort_unstable();
    for (first, second, lettersum) in matches {
        println!(
            "\"{}\" and \"{}\" have no letters in common and a letter sum of {}",
            first, second, lettersum
        );
    }
}

// The list of word `{ geographically, eavesdropper, woodworker, oxymorons }`
// contains 4 words. Each word in the list has both a different number of
// letters, and a different letter sum. The list is sorted both in descending
// order of word length, and ascending order of letter sum. What's the longest
// such list you can find?
fn optional_6(words: &[Word]) {
    let mut words: Vec<Word> = words.to_vec();
    // Sort words primarily by descending length and secondarily by ascending
    // text order.
    words.sort_unstable_by(|first, second| {
        second
            .length
            .cmp(&first.length)
            .then_with(|| first.text.cmp(second.text))
    });

    // Extract the unique letter sums and sort them. This will be used to build
    // the segment tree for efficiently finding the longest sequence of words
    // with increasing letter sums.
    let mut sums: Vec<u32> = words.iter().map(|word| word.letter_sum).collect();
    sums.sort_unstable();
    sums.dedup();

    let mut tree_size = 1;
    while tree_size < sums.len() {
        tree_size *= 2;
    }
    let mut tree = vec![usize::MAX; tree_size * 2];
    let mut lengths = vec![1; words.len()];
    let mut predecessors = vec![usize::MAX; words.len()];
    let mut best_index = 0;

    // Process words in groups of the same length. This ensures that we only
    // compare words of the same length when updating the segment tree.
    let mut start = 0;
    while start < words.len() {
        let mut end = start + 1;
        while end < words.len() && words[end].length == words[start].length {
            end += 1;
        }

        for index in start..end {
            let sum_index = sums.binary_search(&words[index].letter_sum).unwrap();
            let mut left = tree_size;
            let mut right = tree_size + sum_index;
            let mut previous = usize::MAX;
            while left < right {
                if left % 2 == 1 {
                    if tree[left] != usize::MAX
                        && (previous == usize::MAX || lengths[tree[left]] > lengths[previous])
                    {
                        previous = tree[left];
                    }
                    left += 1;
                }
                if right % 2 == 1 {
                    right -= 1;
                    if tree[right] != usize::MAX
                        && (previous == usize::MAX || lengths[tree[right]] > lengths[previous])
                    {
                        previous = tree[right];
                    }
                }
                left /= 2;
                right /= 2;
            }

            if previous != usize::MAX {
                lengths[index] = lengths[previous] + 1;
                predecessors[index] = previous;
            }
            if lengths[index] > lengths[best_index] {
                best_index = index;
            }
        }

        for index in start..end {
            let mut position = tree_size + sums.binary_search(&words[index].letter_sum).unwrap();
            while position < tree.len() {
                if tree[position] == usize::MAX || lengths[index] > lengths[tree[position]] {
                    tree[position] = index;
                }
                position /= 2;
                if position == 0 {
                    break;
                }
            }
        }
        start = end;
    }

    // Reconstruct the longest sequence of words by following the predecessors
    // array.
    let mut longest = Vec::new();
    let mut current = best_index;
    loop {
        longest.push(words[current].text);
        if predecessors[current] == usize::MAX {
            break;
        }
        current = predecessors[current];
    }
    longest.reverse();
    println!(
        "The longest list has {} words: {}",
        longest.len(),
        longest.join(", ")
    );
}
