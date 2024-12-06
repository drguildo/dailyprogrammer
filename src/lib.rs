use std::path::Path;

pub fn file_to_matrix(file: &Path) -> Vec<Vec<String>> {
    let s = std::fs::read_to_string(file).unwrap();
    let matrix = s
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|word| word.to_string())
                .collect::<Vec<String>>()
        })
        .collect();

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_to_matrix() {
        let matrix = file_to_matrix(Path::new("data/test/file_to_matrix.txt"));
        assert_eq!(matrix, [["A", "B", "C",], ["D", "E", "F",],]);
    }
}
