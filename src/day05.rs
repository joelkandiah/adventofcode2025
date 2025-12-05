use std::fs;

pub fn run() {
    println!("Running Day 05 challenge...");
    
    // Read the file content
    let input = fs::read_to_string("inputs/input_day05.txt")
        .expect("Failed to read input file: inputs/input_day05.txt");
    
    // Split the input into two parts based on the blank line separator
    let parts: Vec<&str> = input.split("\n\n").collect();
    
    // Check for exactly two parts
    assert!(
        parts.len() == 2, 
        "Error: Input file should contain exactly two parts separated by a blank line (\\n\\n). Found {} parts.", parts.len()
    );

    // Parse the range lines from the first part
    let extracted_ranges: Vec<(u64,u64)> = parts[0]
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|term| parse_ranges(term).expect("Failed to parse range term"))
        .collect();

    // Parse the individual values from the second part
    let values: Vec<u64> = parts[1]
        .lines()
        .filter(|line| !line.trim().is_empty()) // Ignore empty lines
        .map(|line| {
            line.trim()
                .parse::<u64>()
                .expect(&format!("Failed to parse line '{}' as u64", line))
        })
        .collect();

    // OPTIMIZATION: Move ownership of extracted_ranges to consolidate_ranges 
    // to avoid an unnecessary clone.
    let consolidated_ranges: Vec<(u64, u64)> = consolidate_ranges(extracted_ranges);
    
    // Solve
    println!("Solution Part 1: {}", solve_part1(&consolidated_ranges, &values));
    println!("Solution Part 2: {}", solve_part2(&consolidated_ranges));
}

// Function to parse a single range string (e.g., "10-20")
fn parse_ranges(term: &str) -> Result<(u64,u64), String> {
    let trimmed_term = term.trim();
    if trimmed_term.is_empty() {
        return Err("Empty line encountered during range parsing".to_string());
    }
    
    // Use split_once for robust parsing of exactly one separator
    let (left_str, right_str) = trimmed_term.split_once('-')
        .ok_or_else(|| format!("Expected exactly one '-' in term: {}", term))?;
    
    // Parse left boundary
    let left = left_str.trim().parse::<u64>()
        .map_err(|e| format!("Failed to parse left part '{}' as u64: {}", left_str.trim(), e))?;
    
    // Parse right boundary
    let right = right_str.trim().parse::<u64>()
        .map_err(|e| format!("Failed to parse right part '{}' as u64: {}", right_str.trim(), e))?;

    Ok((left, right))
}

// Pre-processes the overlapping, unsorted ranges into a minimal list
// of non-overlapping, sorted ranges.
fn consolidate_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // A. Sort the ranges by their start point. O(R log R)
    ranges.sort_unstable_by_key(|r| r.0);

    let mut consolidated = Vec::new();
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    // B. Merge overlapping ranges. O(R)
    for i in 1..ranges.len() {
        let (next_start, next_end) = ranges[i];

        // Check if the next range overlaps or touches the current consolidated range
        if next_start <= current_end + 1 { // +1 for touching (e.g., [1,4] and [5,10] merge to [1,10])
            // Overlap: Extend the end
            current_end = current_end.max(next_end);
        } else {
            // No overlap: Push the current consolidated range and start a new one
            consolidated.push((current_start, current_end));
            current_start = next_start;
            current_end = next_end;
        }
    }

    // Push the very last consolidated range
    consolidated.push((current_start, current_end));
    consolidated
}

// Checks if n is in any of the consolidated, non-overlapping ranges using binary search.
// Time complexity: O(log R_new) per query.
fn is_in_consolidated_range(n: &u64, consolidated_ranges: &[(u64, u64)]) -> bool {
    // Binary Search: Find the insertion point (index 'i' where n would go), based on start points.
    let index = match consolidated_ranges.binary_search_by_key(n, |r| r.0) {
        
        // Case 1: Exact match on range start means n is contained.
        Ok(_) => return true,
        
        // Case 2/3: No exact match. Err(i) is the insertion index.
        Err(i) => {
            if i == 0 {
                // n is smaller than all range starts (not contained).
                return false;
            }
            // The potentially containing range is the one *immediately before* the insertion point.
            i - 1
        }
    };

    // Final Check: We know n >= start of range[index]. Check n <= end.
    let (_start, end) = consolidated_ranges[index];
    *n <= end
}


// Part 1: Count how many values fall within the consolidated ranges.
// Uses an iterator chain for idiomatic counting.
fn solve_part1(consolidated_ranges: &[(u64,u64)], values: &[u64]) -> u64 {
    values.iter()
        .filter(|value| is_in_consolidated_range(value, consolidated_ranges))
        .count() as u64
}

// Part 2: Calculate the total length of all consolidated ranges.
// Rewritten as an iterator chain using map and sum.
fn solve_part2(consolidated_ranges: &[(u64,u64)])-> u64 {
    consolidated_ranges.iter()
        // Map each range (start, end) to its length: end - start + 1
        .map(|&(start, end)| end - start + 1)
        // Sum all the calculated lengths
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consolidate_ranges() {
        let ranges_input: Vec<(u64, u64)> = vec![
            (10, 20),
            (5, 12),
            (25, 30),
            (21, 24), // Touches 25-30
            (1, 4), // Touches 5-12
        ];
        // Expected consolidated ranges: (1, 20), (21, 30)
        let expected_output: Vec<(u64, u64)> = vec![(1, 20), (21, 30)];

        assert_eq!(consolidate_ranges(ranges_input), expected_output);
    }

    #[test]
    fn test_solve_part1_range_containment() {
        // Consolidated ranges from input: (3, 5), (10, 18), (19, 20) -> NOTE: consolidation should happen first!
        let unsorted_ranges: Vec<(u64, u64)> = vec![
            (3, 5),    // 3-5
            (10, 14),  // 10-14
            (16, 20),  // 16-20
            (12, 18),  // 12-18
        ];
        
        let consolidated = consolidate_ranges(unsorted_ranges);
        // Consolidated result: [(3, 5), (10, 20)]

        let values_input: Vec<u64> = vec![
            1,   // No
            5,   // YES (in 3-5)
            8,   // No
            11,  // YES (in 10-20)
            17,  // YES (in 10-20)
            32,  // No
        ];
        
        // Contained values: 5, 11, 17 -> Total count is 3
        let expected_result = 3; 

        assert_eq!(solve_part1(&consolidated, &values_input), expected_result);
    }


    #[test]
    fn test_solve_part2() {
        // Ranges: (3, 5), (10, 14), (16, 20), (12, 18)
        let ranges_input: Vec<(u64, u64)> = vec![
            (3, 5),
            (10, 14),
            (16, 20),
            (12, 18),
        ];
        
        let consolidated_ranges: Vec<(u64,u64)> = consolidate_ranges(ranges_input);
        // Consolidated ranges: [(3, 5), (10, 20)]
        // Lengths: (5 - 3 + 1) = 3
        //          (20 - 10 + 1) = 11
        // Total sum = 3 + 11 = 14
        
        let expected_result = 14; 

        assert_eq!(solve_part2(&consolidated_ranges), expected_result);
    }
}
