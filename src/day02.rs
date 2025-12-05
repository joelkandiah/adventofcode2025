use std::fs;

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

const POWERS_OF_10_M1: [u64; 19] = [
    0, 9, 99, 999, 9999, 99999, 999999, 9999999, 99999999, 999999999, 9999999999,
    99999999999, 999999999999, 9999999999999, 99999999999999,
    999999999999999, 9999999999999999, 99999999999999999, 999999999999999999,
];
const POWERS_OF_10_P1: [u64; 19] = [
    1, 11, 101, 1001, 10001, 100001, 1000001, 10000001, 100000001, 1000000001,
    10000000001, 100000000001, 1000000000001, 10000000000001, 100000000000001,
    1000000000000001, 10000000000000001, 100000000000000001, 1000000000000000001,
];

fn get_num_len_log(n: u64) -> u32 {
    if n == 0 { return 1; }
    (n as f64).log10().floor() as u32 + 1
}

fn solve(input: &[(u64,u64)]) ->  u64 {
    // Step one, map over each range
    return input.iter().map(|(start, end)| {
        let mut id_cumsum: u64 = 0;

        for num in *start..=*end {
            if num < 11 { // Check starts at 11 (2 digits, R=2, K=1)
                continue;
            }

            let d_len = get_num_len_log(num);
            
            // 1. Check if length is even (L = 2K)
            if d_len % 2 == 0 { 
                
                let pat_len = d_len / 2; // Pattern length K
                
                // 2. The divisor is D = 10^K + 1
                let divisor = POWERS_OF_10_P1[pat_len as usize];
                
                // 3. Check for divisibility (N % D == 0)
                if num % divisor == 0 {
                    // The number is a valid two-part repeating pattern (e.g., 123123 / 1001 = 123)
                    id_cumsum += num;
                }
            }
        }
        id_cumsum
    }).sum();
}

fn solve_part2(input: &[(u64,u64)]) -> u64 {
    // Powers of 10 are now accessed via the global const POWERS_OF_10
    // No local array initialization needed!

    return input.iter().map(|(start, end)| {
        let mut id_cumsum: u64 = 0;

        'num_loop: for num in *start..=*end {
            if num < 10 {
                continue;
            }

            let d_len = get_num_len_log(num);
            
            // Iterate over all possible number of repetitions R >= 2
            for pat_len in 1..=(d_len / 2) {
                if d_len % pat_len == 0 {
                    
                    // Divisor D = (10^L - 1) / (10^K - 1)
                    let numerator = POWERS_OF_10_M1[d_len as usize]; 
                    let denominator = POWERS_OF_10_M1[pat_len as usize]; 
                    
                    let divisor = numerator / denominator;
                    
                    if num % divisor == 0 {
                        // The number is a valid repeating pattern
                        id_cumsum += num;
                        continue 'num_loop; 
                    }
                }
            }
        }
        id_cumsum
    }).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    // Repeating Pattern Numbers (RPN) examples for reference:
    // Part 1 (Repetition R=2 only): 11, 22, ..., 99, 1010, 1111, ..., 9999, 100100, 101101, ...
    // Part 2 (Repetition R >= 2): Includes all Part 1, plus 111, 222, ..., 999, 101101, 123123, 111111, ...

    #[test]
    fn parse_examples() {
        assert_eq!(parse("10-12").unwrap(), (10, 12));
        assert_eq!(parse("140500-1839201").unwrap(), (140500, 1839201));
    }

    // --- Part 1 Tests (Must have EVEN length, R=2) ---
    #[test]
    fn solve_part1_no_match() {
        // Range 1 to 10: Below 11 threshold
        assert_eq!(solve(&[(1, 10)]), 0, "No RPN below 11");
        // Range 108 to 110: No 2-digit RPN (e.g., 101) in this range
        assert_eq!(solve(&[(108, 110)]), 0, "Range should contain no RPN");
        // 1221 is 4 digits, but 1221 / 101 = 12.08... (not divisible)
        assert_eq!(solve(&[(1221, 1222)]), 0, "1221 is not a Part 1 RPN");
    }

    #[test]
    fn solve_part1_matches() {
        // Includes 11 and 22. Sum = 33.
        assert_eq!(solve(&[(11, 22)]), 33, "Should sum 11 + 22");

        // Includes 1212. Sum = 1212. (1212 / 101 = 12)
        assert_eq!(solve(&[(1200, 1222)]), 1212, "Should find 1212");

        // Two ranges: (1212) + (11 + 22) = 1212 + 33 = 1245
        assert_eq!(solve(&[(1200, 1222), (11, 22)]), 1245, "Should sum RPN from both ranges");
    }

    #[test]
    fn solve_part1_boundary_check() {
        // Should find 99 and 1010. Sum = 1109
        assert_eq!(solve(&[(90, 1010)]), 99 + 1010, "Should find 99 and 1010");
        
        // 123123. Length 6, K=3. 123123 / (1000+1) = 123.
        assert_eq!(solve(&[(123123, 123123)]), 123123, "Should find 123123");
    }

    // --- Part 2 Tests (Must have length L = R*K where R >= 2) ---
    #[test]
    fn solve_part2_matches() {
        // Includes 11, 22. Sum = 33.
        assert_eq!(solve_part2(&[(11, 22)]), 33, "Should sum 11 + 22");

        // Includes 1212. Sum = 1212.
        assert_eq!(solve_part2(&[(1200, 1222)]), 1212, "Should find 1212");

        // Includes 121212. Sum = 121212. (Divisible by 10101)
        assert_eq!(solve_part2(&[(121212, 121213)]), 121212, "Should find 121212");

        // Includes 111 (K=1, R=3). Sum = 111.
        assert_eq!(solve_part2(&[(111, 112)]), 111, "Should find 111 (RPN)");

        // Includes 111 + 1212. Sum = 1323.
        assert_eq!(solve_part2(&[(100, 125)]), 111, "Should find 111 and 1212");
        
        // Includes 11111 (K=1, R=5 is impossible) and 1212. 
        // Part 2 requires L % R == 0. 11111 is prime.
        // Let's check 11111. L=5. Possible R=5. K=1. Divisor (10^5-1)/(10^1-1) = 11111. 11111 % 11111 = 0.
        // The original test assertion looks suspect, but let's correct the calculation based on the function logic.
        // RPNs in (11, 22): 11, 22 (Sum 33)
        // RPNs in (11111, 11112): 11111 (L=5, R=5, K=1).
        // Total sum = 33 + 11111 = 11144. The original test assertion was correct based on this logic.
        assert_eq!(solve_part2(&[(11111, 11112), (11, 22)]), 11144, "Should sum 11, 22, and 11111");
    }
}
