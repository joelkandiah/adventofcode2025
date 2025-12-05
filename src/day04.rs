use std::fs;

pub fn run() {
    println!("Running Day 04 challenge...");
    let input = fs::read_to_string("inputs/input_day04.txt")
        .expect("Failed to read input file");
    let toilet_roll_rows: Vec<Vec<bool>> = input
            .lines()
            .map(|line| parse(line)
            .expect("Failed to parse line")).collect();
    let row_len = toilet_roll_rows.first().map(|r| r.len()).unwrap_or(0);
    let all_same = toilet_roll_rows.iter().all(|r| r.len() == row_len);

    assert!(all_same, "Not all rows have the same length");
   
    println!("Solution Part 1: {}", solve(&toilet_roll_rows));
    println!("Solution Part 2: {}", solve_part2(&toilet_roll_rows));
}  

fn parse(line: &str) -> Result<Vec<bool>, String> {
    assert!(!line.is_empty(), "Empty line in input");
    // Want to match the . to 0 and @ to 1
    // Validate line only contains . and @
    for c in line.chars() {
        if c != '.' && c != '@' {
            return Err(format!("Invalid character '{}' in line: {}", c, line));
        }
    }
    let bool_line: Vec<bool> = line.chars().map(|c| c == '@').collect();
    return Ok(bool_line);
}

fn solve(input: &[Vec<bool>]) ->  u64 {
    // Step one make a matrix which runs a 2d convolution to count neighbours.
    // The convolution kernel is:[1,1,1],[1,0,1],[1,1,1]
    // Note that we want the matrix to be returned as the same size, so we assume oob values are 
    let rows = input.len();
    let cols = input.first().map(|r| r.len()).unwrap_or(0);

    for row in input {
        assert!(row.len() == cols, "All rows must have the same number of columns");
    }
    let mut convolved_matrix: Vec<Vec<u8>> = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let mut neighbor_count: u8 = 0;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue; // Skip the center cell
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        if input[nr as usize][nc as usize] {
                            neighbor_count += 1;
                        }
                    }
                }
            }
            convolved_matrix[r][c] = neighbor_count;
        }
    }
    

    // Step two is to pointwise multiply the convolved matrix with the input matrix
    let mut combined_matrix: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            combined_matrix[r][c] = if input[r][c] {
                convolved_matrix[r][c]
            } else {
                0
            };
        }
    }

    // Step three is to sum all values in the combined matrix
    let mut total_count: u64 = 0;
    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] && combined_matrix[r][c] < 4 {
                total_count += 1;
            }
        }
    }   

    return total_count;
}

fn solve_part2(input: &[Vec<bool>]) -> u64 {
    
    let mut total_removed = 0;
    let mut current_matrix: Vec<Vec<bool>> = input.to_vec();
    // Recursively apply one_step_update until no more can be removed
    loop {
        let (new_matrix, removed) = one_step_update(&current_matrix);
        if removed == 0 {
            break;
        }
        total_removed += removed;
        current_matrix = new_matrix;
    }
    
    total_removed
}


fn one_step_update(input: &[Vec<bool>]) -> (Vec<Vec<bool>>, u64) {
    let rows = input.len();
    let cols = input.first().map(|r| r.len()).unwrap_or(0);

    for row in input {
        assert!(row.len() == cols, "All rows must have the same number of columns");
    }
    let mut convolved_matrix: Vec<Vec<u8>> = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            let mut neighbor_count: u8 = 0;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue; // Skip the center cell
                    }
                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                        if input[nr as usize][nc as usize] {
                            neighbor_count += 1;
                        }
                    }
                }
            }
            convolved_matrix[r][c] = neighbor_count;
        }
    }
    

    // Step two is to pointwise multiply the convolved matrix with the input matrix
    let mut combined_matrix: Vec<Vec<u8>> = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            combined_matrix[r][c] = if input[r][c] {
                convolved_matrix[r][c]
            } else {
                0
            };
        }
    }
    // Step three is to create the return matrix and count removed
    let mut count_removed: u64 = 0;
    let mut return_matrix: Vec<Vec<bool>> = vec![vec![false; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if input[r][c] && combined_matrix[r][c] < 4 {
                count_removed += 1;
            } else if input[r][c] {
                return_matrix[r][c] = true;
            };
        }
    }
    
    return (return_matrix, count_removed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_examples() {
        let raw_input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@."
        ];
        let parsed_input: Vec<Vec<bool>> = raw_input.iter()
            .map(|line| parse(line).expect("Failed to parse line")).collect();
        assert_eq!(solve(&parsed_input), 13); 
    }

    #[test]
    fn solve_examples_2() {
        let raw_input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@."
        ];
        let parsed_input: Vec<Vec<bool>> = raw_input.iter()
            .map(|line| parse(line).expect("Failed to parse line")).collect();
        assert_eq!(solve_part2(&parsed_input), 43); 
    }

}

