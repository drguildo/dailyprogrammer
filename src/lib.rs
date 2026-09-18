use std::path::Path;

// This is where helper code that is used throughout the project is placed.

/// Converts the contents of a file into a matrix of strings. Each line in the
/// file becomes a row in the matrix, and each whitespace-separated word becomes
/// an element in the row.
pub fn file_to_matrix<P: AsRef<Path>>(file: P) -> Vec<Vec<String>> {
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
        let matrix = file_to_matrix("data/test/file_to_matrix.txt");
        assert_eq!(matrix, [["A", "B", "C",], ["D", "E", "F",],]);
    }
}
