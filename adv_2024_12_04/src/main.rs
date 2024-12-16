use std::fs;

fn main() {
    let matrix = fill();
    problem_two(matrix);
}

fn fill() -> Vec<Vec<char>> {
    fs::read_to_string("data.txt")
        .unwrap()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn problem_one(matrix: Vec<Vec<char>>) -> i32 {
    let cols = matrix.first().map(|item| item.len()).unwrap();
    let rows = matrix.len();
    let mut total = 0;

    for col in 0..cols {
        for row in 0..rows {
            if matrix[row][col] == 'X' {
                // right
                if (cols - col - 1) > 2 {
                    if matrix[row][col + 1] == 'M'
                        && matrix[row][col + 2] == 'A'
                        && matrix[row][col + 3] == 'S'
                    {
                        total += 1;
                        println!("right {} {}", row, col);
                    }
                }

                // left
                if col > 2 {
                    if matrix[row][col - 1] == 'M'
                        && matrix[row][col - 2] == 'A'
                        && matrix[row][col - 3] == 'S'
                    {
                        total += 1;
                        println!("left {} {}", row, col);
                    }
                }

                // down
                if (rows - row - 1) > 2 {
                    if matrix[row + 1][col] == 'M'
                        && matrix[row + 2][col] == 'A'
                        && matrix[row + 3][col] == 'S'
                    {
                        total += 1;
                        println!("down {} {}", row, col);
                    }
                }

                // up
                if row > 2 {
                    if matrix[row - 1][col] == 'M'
                        && matrix[row - 2][col] == 'A'
                        && matrix[row - 3][col] == 'S'
                    {
                        total += 1;
                        println!("up {} {}", row, col);
                    }
                }

                // right down
                if (rows - row - 1) > 2 && (cols - col - 1) > 2 {
                    if matrix[row + 1][col + 1] == 'M'
                        && matrix[row + 2][col + 2] == 'A'
                        && matrix[row + 3][col + 3] == 'S'
                    {
                        total += 1;
                        println!("right down {} {}", row, col);
                    }
                }

                // left down
                if (rows - row - 1) > 2 && col > 2 {
                    if matrix[row + 1][col - 1] == 'M'
                        && matrix[row + 2][col - 2] == 'A'
                        && matrix[row + 3][col - 3] == 'S'
                    {
                        total += 1;
                        println!("left down {} {}", row, col);
                    }
                }

                // right up
                if row > 2 && (cols - col - 1) > 2 {
                    if matrix[row - 1][col + 1] == 'M'
                        && matrix[row - 2][col + 2] == 'A'
                        && matrix[row - 3][col + 3] == 'S'
                    {
                        total += 1;
                        println!("right up {} {}", row, col);
                    }
                }

                // left up
                if row > 2 && col > 2 {
                    if matrix[row - 1][col - 1] == 'M'
                        && matrix[row - 2][col - 2] == 'A'
                        && matrix[row - 3][col - 3] == 'S'
                    {
                        total += 1;
                        println!("left up {} {}", row, col);
                    }
                }
            }
        }
    }
    total
}

fn problem_two(matrix: Vec<Vec<char>>) -> i32 {
    let cols = matrix.first().map(|item| item.len()).unwrap();
    let rows = matrix.len();
    let mut total = 0;

    for col in 0..cols {
        for row in 0..rows {
            if matrix[row][col] == 'A' {
                //
                let can_move_left_up = col > 0 && row > 0;
                let can_move_right_down = (rows - row - 1) > 0 && (cols - col - 1) > 0;

                let can_move_right_up = (cols - col - 1) > 0 && row > 0;
                let can_move_left_down = col > 0 && (rows - row - 1) > 0;

                if !can_move_left_up
                    || !can_move_right_down
                    || !can_move_right_up
                    || !can_move_left_down
                {
                    continue;
                }

                if matrix[row - 1][col - 1] == 'M'
                    && matrix[row + 1][col + 1] == 'S'
                    && matrix[row - 1][col + 1] == 'S'
                    && matrix[row + 1][col - 1] == 'M'
                {
                    total += 1;
                }

                if matrix[row - 1][col - 1] == 'M'
                    && matrix[row + 1][col + 1] == 'S'
                    && matrix[row - 1][col + 1] == 'M'
                    && matrix[row + 1][col - 1] == 'S'
                {
                    total += 1;
                }

                if matrix[row - 1][col - 1] == 'S'
                    && matrix[row + 1][col + 1] == 'M'
                    && matrix[row - 1][col + 1] == 'M'
                    && matrix[row + 1][col - 1] == 'S'
                {
                    total += 1;
                }

                if matrix[row - 1][col - 1] == 'S'
                    && matrix[row + 1][col + 1] == 'M'
                    && matrix[row - 1][col + 1] == 'S'
                    && matrix[row + 1][col - 1] == 'M'
                {
                    total += 1;
                }
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_match() {
        let matrix = vec![
            vec!['.', '.', '.', '.', 'X', 'M', 'A', 'S', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 1);
    }

    #[test]
    fn test_vertical_match() {
        let matrix = vec![
            vec!['.', '.', '.', 'X', '.'],
            vec!['.', '.', '.', 'M', '.'],
            vec!['.', '.', '.', 'A', '.'],
            vec!['.', '.', '.', 'S', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 1);
    }

    #[test]
    fn test_diagonal_match_right_down() {
        let matrix = vec![
            vec!['X', '.', '.', '.', '.'],
            vec!['.', 'M', '.', '.', '.'],
            vec!['.', '.', 'A', '.', '.'],
            vec!['.', '.', '.', 'S', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 1);
    }

    #[test]
    fn test_diagonal_match_left_down() {
        let matrix = vec![
            vec!['.', '.', '.', '.', 'X'],
            vec!['.', '.', '.', 'M', '.'],
            vec!['.', '.', 'A', '.', '.'],
            vec!['.', 'S', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 1);
    }

    #[test]
    fn test_no_match() {
        let matrix = vec![
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 0);
    }

    #[test]
    fn test_multiple_directions() {
        let matrix = vec![
            vec!['X', 'M', 'A', 'S', '.', '.', '.', '.', '.'],
            vec!['M', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['A', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['S', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(problem_one(matrix), 2);
    }

    #[test]
    fn test_problem_one() {
        let matrix = fill();
        assert_eq!(problem_one(matrix), 2560);
    }

    #[test]
    fn test_problem_two() {
        let matrix = fill();
        assert_eq!(problem_two(matrix), 1910);
    }
}
