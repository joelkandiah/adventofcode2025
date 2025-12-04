use std::fs;
use std::collections::HashSet;

pub fn run() {
    println!("Running Day 02 challenge...");
    // Add your Day 02 challenge code here
    let input = fs::read_to_string("inputs/input_day02.txt")
        .expect("Failed to read input file");
    let extracted_ranges: Vec<(u64,u64)> = input.split(',')
        .map(|term| parse(term).expect("Failed to parse term"))
        .collect();
    println!("Solution Part 1: {}", solve(&extracted_ranges));
    println!("Solution Part 2: {}", solve_part2(&extracted_ranges));
}  

fn parse(term: &str) -> Result<(u64,u64), String> {
    // Ensure the line is not empty
    assert!(!term.is_empty(), "Empty line in input");
    let parts: Vec<&str> = term.trim().split('-').collect();
    
    assert!(parts.len() == 2, "Expected two parts in term: {}", term);
   
    let left = parts[0].parse::<u64>()
        .map_err(|_| format!("Failed to parse left part '{}' as u64", parts[0]))?;
    let right = parts[1].parse::<u64>()
        .map_err(|_| format!("Failed to parse right part '{}' as u64", parts[1]))?;

    Ok((left, right))
}


fn solve(input: &[(u64,u64)]) ->  u64 {
    // Step one, map over each range
    return input.iter().map(|(start, end)| {
        let mut id_cumsum: u64 = 0;   
        // For each range, filter only the valid numbers 
        for num in *start..=*end {
            let digits: Vec<char> = num.to_string().chars().collect();
            
            let d_len = digits.len();
            // Check non-decreasing condition
            if d_len % 2 != 0 {
                continue; // Skip numbers that are not even
            }
            
            // Check if digits are duplicated                     
            if &digits[..(d_len/2)] == &digits[(d_len/2)..]{
                id_cumsum += num;
            }
        }
        return id_cumsum;
    }).sum();
}

fn solve_part2(input: &[(u64,u64)]) ->  u64 {
    // Step one, map over each range
    return input.iter().map(|(start, end)| {
        let mut id_cumsum: u64 = 0;   
        // For each range, filter only the valid numbers 
        'num_loop: for num in *start..=*end {
            let digits: Vec<char> = num.to_string().chars().collect();
            
            let d_len = digits.len();
            if d_len < 2 {
                continue; // Skip numbers that are not even
            }
            // Loop over numbers that divide length
            for reps in 2..=d_len/2 {
                if d_len % reps != 0 {
                    continue;
                }
                let pat_len = (d_len / reps) as usize;
                let pattern = &digits[..pat_len];
                // Check if digits are duplicated num times                    
                if digits.chunks(pat_len).all(|my_chunk| my_chunk == pattern) {
                    id_cumsum += num;
                    continue 'num_loop; // No need to check further patterns if one is found
                }
            }
            
            // Check case where length >= 2 and all digits are equal
            if digits.into_iter().collect::<HashSet<char>>().len() == 1 {
                id_cumsum += num;
            }
        }
        return id_cumsum;
    }).sum();
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_examples() {
        assert_eq!(parse("10-12").unwrap(), (10, 12));
        assert_eq!(parse("140500-1839201").unwrap(), (140500, 1839201));
    }

    #[test]
    fn solve_examples() {
        assert_eq!(solve(&[(1, 2)]), 0); // example: sum
        assert_eq!(solve(&[(108,110)]), 0); // single element slice
        assert_eq!(solve(&[(11, 22)]), 33); // 50 → 100 mod 100 = 0
        assert_eq!(solve(&[(1221,1222)]), 0); // 50 → 100 mod 100 = 0
        assert_eq!(solve(&[(1212,1222)]), 1212); // 50 → 100 mod 100 = 0
        assert_eq!(solve(&[(1200,1222)]), 1212); // 50 → 100 mod 100 = 0
        assert_eq!(solve(&[(1221,1222), (11,22)]), 33); // 50 → 100 mod 100 = 0
    }
    #[test]
    fn solve_examples_part2() {
        assert_eq!(solve_part2(&[(1, 2)]), 0); // example: sum
        assert_eq!(solve_part2(&[(108,110)]), 0); // single element slice
        assert_eq!(solve_part2(&[(11, 22)]), 33); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(1221,1222)]), 0); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(1212,1222)]), 1212); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(121212,121213)]), 121212); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(121212,122222)]), 243334); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(1200,1222)]), 1212); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(1221,1222), (11,22)]), 33); // 50 → 100 mod 100 = 0
        assert_eq!(solve_part2(&[(11111,11112), (11,22)]), 11144); // 50 → 100 mod 100 = 0
        }
}

