fn main() {
    if let Some(strand) = std::env::args().nth(1) {
        if let Some(complement) = complementary_strand(&strand) {
            println!("{}", strand);
            println!("{}", complement);
        } else {
            eprintln!("Failed to find complementary strand for {}.", strand);
        }
    } else {
        eprintln!("Please specify a DNA strand.")
    }
}

fn corresponding_base(base: char) -> Option<char> {
    match base {
        ' ' => Some(' '),
        'a' => Some('T'),
        'A' => Some('T'),
        't' => Some('A'),
        'T' => Some('A'),
        'c' => Some('G'),
        'C' => Some('G'),
        'g' => Some('C'),
        'G' => Some('C'),
        _ => None,
    }
}

fn complementary_strand<S: AsRef<str>>(strand: S) -> Option<String> {
    strand.as_ref().chars().map(corresponding_base).collect()
}

#[cfg(test)]
mod tests {
    use crate::{complementary_strand, corresponding_base};

    #[test]
    fn test_corresponding_base() {
        assert_eq!(corresponding_base(' '), Some(' '));

        assert_eq!(corresponding_base('a'), Some('T'));
        assert_eq!(corresponding_base('t'), Some('A'));

        assert_eq!(corresponding_base('c'), Some('G'));
        assert_eq!(corresponding_base('g'), Some('C'));

        assert_eq!(corresponding_base('x'), None);
    }

    #[test]
    fn test_complementary_strand() {
        assert_eq!(
            complementary_strand("A A T G C C T A T G G C"),
            Some("T T A C G G A T A C C G".to_string())
        );
    }
}
