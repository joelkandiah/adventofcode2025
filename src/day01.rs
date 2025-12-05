use std::fs;

pub fn run() {
    println!("Running Day 01 challenge...");
    // Add your Day 01 challenge code here
    let input = fs::read_to_string("inputs/input_day01.txt").expect("Failed to read input file");
    let directions: Vec<i16> = input.lines().map(parse).collect();
    println!("Solution Part 1: {}", solve(&directions, &50, &100));
    println!("Solution Part 2: {}", solve_part2(&directions, &50, &100));
}  

fn parse(line: &str) -> i16 {
    // Ensure the line is not empty
    assert!(!line.is_empty(), "Empty line in input");

    // First character must be L or R
    let dir = line.chars().next().expect("Missing direction");
    assert!(dir == 'L' || dir == 'R', "Invalid direction: {}", dir);
    let magnitude: u16 = line[1..].parse().expect("Invalid Number or out of i8 range");
    

    match dir {
        'L' => -(magnitude as i16),
        'R' => magnitude as i16,
        // Note: unreachable as other directions are thrown as Invalid
        _ => unreachable!(),
    }
}



fn solve(input: &[i16], start_position: &u16, dial_size: &u16) -> u16 {
    // Count how many times the dial is at zero
    // start position should be between 0 and dial_size - 1
    assert!(start_position < dial_size, "Start position ({}) must be less than dial size ({})", start_position, dial_size);

    // Initialize counters
    let mut cumulative_sum: i16 = *start_position as i16;
    let mut zero_count: u16 = 0;

    // Loop through the instructions and check when it hits zero
    // Note we also need to use that the dial loops i.e. 0 - 10 == dial_size - 10
    for &value in input {
        cumulative_sum += value;
        // Note this needs to be fixed as -1 % dial_size should be dial_size -1  not -1
        cumulative_sum = cumulative_sum.rem_euclid(*dial_size as i16);
        if cumulative_sum == 0 {
            zero_count += 1;
        }
    }
    
    return zero_count;
}

fn solve_part2(input: &[i16], start_position: &u16, dial_size: &u16) -> u16 {
    let d = *dial_size as i16;
    let mut unwrapped_pos: i16 = *start_position as i16;
    let mut zero_count: u16 = 0;

    // Loop through the instructions and check how many times it clicks to  zero
    for &value in input.iter() {
        let start_pos = unwrapped_pos;
        let end_pos = unwrapped_pos + value;

        let k_start = start_pos.div_euclid(d);
        let k_end = end_pos.div_euclid(d);

        unwrapped_pos = end_pos.rem_euclid(d);

        let mut crosses = (k_end - k_start).abs() as u16;

        // Adjust for edge cases where we start or end exactly on zero or don't "cross" zero
        if crosses > 0 && start_pos == 0 && value < 0{
            crosses -= 1;
        } else if crosses == 0 && unwrapped_pos == 0 {
            crosses += 1;
        }

        if k_end < 0 && unwrapped_pos == 0 {
            crosses += 1;
        }

        zero_count += crosses;
    }

    return zero_count;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_examples() {
        assert_eq!(parse("L5"), -5);
        assert_eq!(parse("R10"), 10);
        assert_eq!(parse("L0"), 0);
        assert_eq!(parse("R100"), 100);
    }

    #[test]
    fn solve_examples() {
        assert_eq!(solve(&[1, -1], &0, &100), 1); // example: sum
        assert_eq!(solve(&[-18], &0, &100), 0); // single element slice
        assert_eq!(solve(&[50, 50], &0, &100), 1); // 50 → 100 mod 100 = 0
        assert_eq!(solve(&[50, 150], &0, &100), 1); // 50 → 100 mod 100 = 0
    }

#[test]
    fn solve_examples_part2() {
        assert_eq!(solve_part2(&[1, -1], &0, &100), 1); // example: sum
        assert_eq!(solve_part2(&[-1, 1], &0, &100), 1); // example: sum
        assert_eq!(solve_part2(&[-18], &0, &100), 0); // single element slice
        assert_eq!(solve_part2(&[50, 150], &0, &100), 2); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[50, -150], &0, &100), 2); // example: sum
    }
}

