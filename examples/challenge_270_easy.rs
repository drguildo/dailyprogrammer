fn main() {
    let matrix: Vec<Vec<char>> = std::fs::read_to_string("data/challenge_270_easy.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let transposed = transpose(&matrix);
    print_matrix(&transposed);
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let longest = matrix
        .iter()
        .fold(0, |acc, v| if v.len() > acc { v.len() } else { acc });

    let mut transposed: Vec<Vec<char>> = Vec::new();
    for y in 0..matrix.len() {
        for x in 0..longest {
            if x >= transposed.len() {
                transposed.push(Vec::new());
            }
            let c = if matrix[y].len() > x {
                matrix[y][x]
            } else {
                ' '
            };
            transposed[x].push(c);
        }
    }

    transposed
}

fn print_matrix(matrix: &Vec<Vec<char>>) {
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            print!("{}", matrix[y][x]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let matrix: Vec<Vec<char>> = vec!["Some".chars().collect(), "Text.".chars().collect()];
        let transposed = transpose(&matrix);
        assert_eq!(
            transposed,
            [['S', 'T'], ['o', 'e'], ['m', 'x'], ['e', 't'], [' ', '.']]
        );
    }
}
