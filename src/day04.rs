use std::fs;

pub fn run() {
    println!("Running Day 04 challenge...");
    let input = fs::read_to_string("inputs/input_day04.txt")
        .expect("Failed to read input file");
    let (toilet_rolls, rows, cols): (Vec<bool>, usize, usize) = parse(&input).expect("Failed to parse");
   
    println!("Solution Part 1: {}", solve(&toilet_rolls, rows, cols));
    println!("Solution Part 2: {}", solve_part2(&toilet_rolls, rows, cols));
}  

fn parse(input: &str) -> Result<(Vec<bool>, usize, usize), String> {
    let mut data = Vec::new();
    let mut rows = 0;
    let mut cols = 0;
    
    for line in input.lines().filter(|l| !l.is_empty()) {
        rows += 1;
        let line_len = line.len();

        // Check for uniform column length
        if cols == 0 {
            cols = line_len;
        } else if line_len != cols {
            return Err(format!("Row {} has length {} but expected {}", rows, line_len, cols));
        }

        for c in line.chars() {
            match c {
                '@' => data.push(true),
                '.' => data.push(false),
                _ => return Err(format!("Invalid character '{}' in line: {}", c, line)),
            }
        }
    }

    Ok((data, rows, cols))
}

fn solve(input: &[bool], rows: usize, cols: usize) ->  u64 {
    // Step one make a matrix which runs a 2d convolution to count neighbours.
    // The convolution kernel is:[1,1,1],[1,0,1],[1,1,1]
    // Note that we want the matrix to be returned as the same size, so we assume oob values are 

    let mut convolved_matrix: Vec<u8> = vec![0; rows * cols];

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
                        let (n_r, n_c) = (nr as usize, nc as usize);
                        if input[n_r * cols + n_c] {
                            neighbor_count += 1;
                        }
                    }
                }
            }
            convolved_matrix[r * cols + c] = neighbor_count;
        }
    }
    

    // Step two is to pointwise multiply the convolved matrix with the input matrix
    let mut combined_matrix: Vec<u8> = vec![0; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            combined_matrix[r * cols + c] = if input[r * cols + c] {
                convolved_matrix[r * cols + c]
            } else {
                0
            };
        }
    }

    // Step three is to sum all values in the combined matrix
    let mut total_count: u64 = 0;
    for r in 0..rows {
        for c in 0..cols {
            if input[r * cols + c] && combined_matrix[r * cols + c] < 4 {
                total_count += 1;
            }
        }
    }   

    return total_count;
}

fn solve_part2(input: &[bool], rows: usize, cols: usize) -> u64 {
    
    let mut total_removed = 0;
    let mut current_matrix  = input.to_vec();
    let mut new_matrix  = vec![false; rows * cols];
    // Recursively apply one_step_update until no more can be removed
    loop {
        let removed = one_step_update(&current_matrix, &mut new_matrix, rows, cols);
        if removed == 0 {
            break;
        }
        total_removed += removed;
        std::mem::swap(&mut current_matrix, &mut new_matrix);
        new_matrix.fill(false);
    }
    
    total_removed
}


fn one_step_update(input: &[bool], return_matrix: &mut [bool], rows: usize, cols: usize) -> u64 {

    let mut convolved_matrix: Vec<u8> = vec![0; rows * cols];

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
                        let (n_r, n_c) = (nr as usize, nc as usize);
                        if input[n_r * cols + n_c] {
                            neighbor_count += 1;
                        }
                    }
                }
            }
            convolved_matrix[r * cols + c] = neighbor_count;
        }
    }
    

    // Step two is to pointwise multiply the convolved matrix with the input matrix
    let mut combined_matrix: Vec<u8> = vec![0; rows * cols];
    for r in 0..rows {
        for c in 0..cols {
            combined_matrix[r * cols + c] = if input[r * cols + c] {
                convolved_matrix[r * cols + c]
            } else {
                0
            };
        }
    }

    let mut count_removed: u64 = 0;
    // Step three is to create the return matrix and count removed
    for r in 0..rows {
        for c in 0..cols {
            if input[r * cols + c] && combined_matrix[r * cols + c] < 4 {
                return_matrix[r * cols + c] = false;
                count_removed += 1;
            } else if input[r * cols + c] {
                return_matrix[r * cols + c] = true;
            };
        }
    }
    
    return count_removed;
}

#[cfg(test)]
mod tests {
    use super::*;

    // This is the example input for both parts
    const EXAMPLE_INPUT: &str = r#"
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"#; // Using a raw string literal for the multi-line input

    #[test]
    fn solve_examples_part1() {
        // 1. Call parse once with the full input string
        let (toilet_rolls, rows, cols) = parse(EXAMPLE_INPUT)
            .expect("Failed to parse example input");

        // 2. Call solve with the correct arguments
        let result = solve(&toilet_rolls, rows, cols);
        
        // The expected result for Part 1 based on similar problems (e.g., Conway's Game of Life rules)
        // Note: The expected value 13 is assumed to be correct based on the original broken test.
        assert_eq!(result, 13);
    }

    #[test]
    fn solve_examples_part2() {
        // 1. Call parse once with the full input string
        let (toilet_rolls, rows, cols) = parse(EXAMPLE_INPUT)
            .expect("Failed to parse example input");

        // 2. Call solve_part2 with the correct arguments
        let result = solve_part2(&toilet_rolls, rows, cols);
        
        // Note: The expected value 43 is assumed to be correct based on the original broken test.
        assert_eq!(result, 43);
    }

    #[test]
    fn parse_examples_correct_dimensions() {
        let (toilet_rolls, rows, cols) = parse(EXAMPLE_INPUT)
            .expect("Failed to parse example input");

        // The example is a 10x10 grid
        assert_eq!(rows, 10);
        assert_eq!(cols, 10);
        assert_eq!(toilet_rolls.len(), 10 * 10);
    }
}
