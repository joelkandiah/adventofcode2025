# Advent of Code 2025

A Rust-based solution framework for Advent of Code 2025 with CLI support, benchmarking, and automatic scaffolding.

## Quick Start

### Running Solutions

Run a specific day:
```bash
cargo run --bin aoc2025 -- 4
```

Run with benchmarking (dev mode):
```bash
cargo run --bin aoc2025 -- 4 --bench
```

**Benchmark with optimizations (recommended for accurate timing):**
```bash
cargo run --release --bin aoc2025 -- 4 --bench
```

Run all implemented days:
```bash
cargo run --bin aoc2025 -- --all
```

Run all days with benchmarking:
```bash
cargo run --bin aoc2025 -- --all --bench
```

### Testing

Test all days:
```bash
cargo test
```

Test a specific day (filter by module name):
```bash
cargo test day02
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Test a specific day with output:
```bash
cargo test day02 -- --nocapture
```

### Benchmarking

For accurate performance measurements, always use `--release` mode:

```bash
# Benchmark a single day (optimized)
cargo run --release --bin aoc2025 -- 4 --bench

# Benchmark all days (optimized)
cargo run --release --bin aoc2025 -- --all --bench
```

**Dev vs Release mode:**
- **Dev mode** (`cargo run`): Fast compilation, slow execution (~10-100x slower)
- **Release mode** (`cargo run --release`): Slow compilation, fast execution with full optimizations

> **Tip:** Use dev mode for testing and debugging, release mode for benchmarking and final solutions.

### Creating a New Day

Automatically scaffold a new day (creates file, input, and updates main.rs):
```bash
cargo run --bin scaffold -- 5
```

This will:
- Create `src/day05.rs` from a template
- Create `inputs/input_day05.txt` (empty, ready for your input)
- Update `src/main.rs` with the module declaration and dispatch logic

## Project Structure

```
aoc2025/
├── src/
│   ├── main.rs           # CLI entry point with day dispatcher
│   ├── bin/
│   │   └── scaffold.rs   # Scaffolding tool for new days
│   ├── day01.rs          # Day 1 solution
│   ├── day02.rs          # Day 2 solution
│   └── ...
├── inputs/
│   ├── input_day01.txt   # Day 1 puzzle input
│   ├── input_day02.txt   # Day 2 puzzle input
│   └── ...
└── Cargo.toml
```

## Day File Template

Each day file follows this structure:

```rust
use std::fs;

pub fn run() {
    println!("Running Day XX challenge...");
    let input = fs::read_to_string("inputs/input_dayXX.txt")
        .expect("Failed to read input file");
    
    // Parse input
    let lines: Vec<&str> = input.lines().collect();
    
    // Solve
    println!("Solution Part 1: {}", solve_part1(&lines));
    println!("Solution Part 2: {}", solve_part2(&lines));
}

fn solve_part1(_input: &[&str]) -> u64 {
    0
}

fn solve_part2(_input: &[&str]) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![];
        assert_eq!(solve_part1(&input), 0);
    }
}
```

## Tips

- Keep your parsing and solving logic separate for clarity
- Write tests using the example inputs from the problem statement
- Use `--bench` to optimize your solutions
- The scaffold tool automatically updates `main.rs`, so you don't need to manually add new days

## Dependencies

- `clap` - Command-line argument parsing
- `colored` - Terminal output styling (reserved for future use)
