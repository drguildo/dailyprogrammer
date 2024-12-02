use std::collections::HashSet;

fn main() {
    let s = "abcbbbbcccbdddadacb";
    let result = find_longest(s);
    println!("{}", result);
}

fn find_longest(s: &str) -> &str {
    if s.len() < 3 {
        return s;
    }

    let mut longest = &s[0..2];
    let mut search_start = 0;
    let mut search_len = 3;

    while s[search_start..].len() >= search_len {
        let candidate = &s[search_start..search_start + search_len];
        if count_unique(candidate) <= 2 {
            longest = candidate;
            search_len += 1;
        } else {
            search_start += 1;
        }
    }

    longest
}

fn count_unique(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }

    let mut unique_chars = HashSet::new();

    for c in s.chars() {
        unique_chars.insert(c);
    }

    unique_chars.len()
}

#[cfg(test)]
mod tests {
    use super::count_unique;
    use super::find_longest;

    #[test]
    fn test_abcbbbbcccbdddadacb() {
        assert_eq!(find_longest("abcbbbbcccbdddadacb"), "bcbbbbcccb");
    }

    #[test]
    fn test_abbccc() {
        assert_eq!(find_longest("abbccc"), "bbccc");
    }

    #[test]
    fn test_abcabcabcabccc() {
        assert_eq!(find_longest("abcabcabcabccc"), "bccc");
    }

    #[test]
    fn test_qwertyytrewq() {
        assert_eq!(find_longest("qwertyytrewq"), "tyyt");
    }

    #[test]
    fn test_bbbbb() {
        assert_eq!(find_longest("bbbbb"), "bbbbb");
    }

    #[test]
    fn test_ab() {
        assert_eq!(find_longest("ab"), "ab");
    }

    #[test]
    fn test_abc() {
        assert_eq!(find_longest("abc"), "ab");
    }

    #[test]
    fn test_abac() {
        assert_eq!(find_longest("abac"), "aba");
    }

    #[test]
    fn test_a() {
        assert_eq!(find_longest("a"), "a");
    }

    #[test]
    fn test_empty() {
        assert_eq!(find_longest(""), "");
    }

    #[test]
    fn test_count_unique() {
        let s = "abcbbbbcccbdddadacb";
        assert_eq!(count_unique(s), 4);
    }

    #[test]
    fn test_count_unique_empty() {
        let s = "";
        assert_eq!(count_unique(s), 0);
    }

    #[test]
    fn test_count_unique_single() {
        let s = "a";
        assert_eq!(count_unique(s), 1);
    }
}
