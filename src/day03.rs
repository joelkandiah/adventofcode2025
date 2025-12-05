use std::fs;

pub fn run() {
    println!("Running Day 03 challenge...");
    // Add your Day 02 challenge code here
    let input = fs::read_to_string("inputs/input_day03.txt")
        .expect("Failed to read input file");
    let batteries: Vec<&str> = input.lines().collect();
    println!("Solution Part 1: {}", solve(&batteries));
    println!("Solution Part 2: {}", solve_part2(&batteries));
}  


fn solve(input: &[&str]) ->  u64 {
    // Step one, map over each range
    let mut joltage: u64 = 0;
    let mut big1:  u8;
    let mut big2:  u8;
    let mut idx1: u16;
    let mut idx2: u16;
    for battery in input {
        big1 = 0;
        idx1 = 0;
        big2 = 0;
        idx2 = 0;
        for (i, v) in battery.chars().enumerate() {
            let v: u8 = v.to_digit(10).expect(&format!("Char was not a digit {}", battery)) as u8;
            if big1 == big2 && big1 == 9 {
                  break;
            }
            if v > big1 {
                big2 = big1;

                big1 = v;
                idx1 = i as u16;
            } else if v > big2  || idx1 > idx2 {
                big2 = v;
                idx2 = i as u16;
            }
        }
        joltage += if idx1 < idx2 {(big1 as u64) * 10 + (big2 as u64)} else {(big2 as u64) * 10 + (big1 as u64)};
    }

    return joltage;
}

fn solve_part2(input: &[&str]) ->  u64 {
    // Step one, map over each range
    let mut joltage: u64 = 0;
    for battery in input {
        let to_pick = 12;
        let mut stack: Vec<u8> = Vec::new();
        let n = battery.len();
        for (i, v) in battery.chars().enumerate() {
            let v: u8 = v.to_digit(10).expect(&format!("Char was not a digit {}", battery)) as u8;
            let remaining = n - i;
            while let Some(&last) = stack.last() {
                if last < v && remaining + stack.len() > to_pick {
                    stack.pop();
                } else {
                    break;
                }
            }
            // Push current digit if stack not full
            if stack.len() < to_pick {
                stack.push(v);
            }

        }
        joltage += stack.iter().fold(0, |acc, &d| acc * 10 + d as u64); 
    }

    return joltage;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_examples() {
        assert_eq!(solve(&["12788"]), 88); 
        assert_eq!(solve(&["1278"]), 78); 
        assert_eq!(solve(&["9988"]), 99); 
        assert_eq!(solve(&["88989"]), 99); 
        assert_eq!(solve(&["12000"]), 20); 
        assert_eq!(solve(&["12000", "1278"]), 98); 
        assert_eq!(solve(&["12892000"]), 92); 
        assert_eq!(solve(&["987654321111111", "811111111111119", "234234234234278", "818181911112111"]), 357); 
    }
    #[test]
    fn solve_examples_part2() {
        assert_eq!(solve_part2(&["987654321111111", "811111111111119", "234234234234278", "818181911112111"]), 3121910778619); 
    }

}

