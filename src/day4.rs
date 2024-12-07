use advent_of_code_2024::Day;

pub struct Day4;

impl std::fmt::Display for Day4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day4")
    }
}

impl Day for Day4 {
    fn part1(&self) -> usize {
        let input = std::fs::read_to_string("./input/day4").unwrap();

        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        fn count_xmas(matrix: &[Vec<char>], i: usize, j: usize) -> usize {
            let mut result = 0;

            if matrix[i][j] != 'X' {
                return result;
            }

            for k in -1..=1 {
                for l in -1..=1 {
                    let is_xmas = "XMAS".chars().enumerate().skip(1).all(|(m, c)| {
                        let cord_i = i as isize + k * m as isize;
                        if !(0..matrix.len() as isize).contains(&cord_i) {
                            return false;
                        }
                        let cord_j = j as isize + l * m as isize;
                        if !(0..matrix[0].len() as isize).contains(&cord_j) {
                            return false;
                        }
                        matrix[cord_i as usize][cord_j as usize] == c
                    });
                    if is_xmas {
                        result += 1;
                    }
                }
            }

            result
        }

        let mut result = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                result += count_xmas(&matrix, i, j);
            }
        }

        result
    }

    fn part2(&self) -> usize {
        let input = std::fs::read_to_string("./input/day4").unwrap();

        let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        fn where_xmas(matrix: &[Vec<char>], i: usize, j: usize) -> bool {
            if matrix[i][j] != 'A' {
                return false;
            }
            if i == 0 || i == matrix.len() - 1 || j == 0 || j == matrix[0].len() - 1 {
                return false;
            }

            let slash: String = [matrix[i - 1][j - 1], matrix[i][j], matrix[i + 1][j + 1]]
                .iter()
                .collect();
            let backslash: String = [matrix[i - 1][j + 1], matrix[i][j], matrix[i + 1][j - 1]]
                .iter()
                .collect();

            (slash == "MAS" || slash == "SAM") && (backslash == "MAS" || backslash == "SAM")
        }

        let mut result = 0;

        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if where_xmas(&matrix, i, j) {
                    result += 1;
                }
            }
        }

        result
    }
}
