use clap::Parser;
use std::fs;
// use std::io::Write;
use std::path::Path;

#[derive(Parser)]
struct Args {
    day: u8,
}

fn main() {
    let args = Args::parse();
    let day = args.day;
    let day_str = format!("day{:02}", day);
    let file_path = format!("src/{}.rs", day_str);

    if Path::new(&file_path).exists() {
        eprintln!("File {} already exists!", file_path);
        return;
    }

    // 1. Create the day file
    let template = format!(r#"use std::fs;

pub fn run() {{
    println!("Running Day {:02} challenge...");
    let input = fs::read_to_string("inputs/input_{}.txt")
        .expect("Failed to read input file");
    
    // Parse input
    let lines: Vec<&str> = input.lines().collect();
    
    // Solve
    println!("Solution Part 1: {{}}", solve_part1(&lines));
    println!("Solution Part 2: {{}}", solve_part2(&lines));
}}

fn solve_part1(_input: &[&str]) -> u64 {{
    0
}}

fn solve_part2(_input: &[&str]) -> u64 {{
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part1() {{
        let input = vec![];
        assert_eq!(solve_part1(&input), 0);
    }}
}}
"#, day, day_str);

    fs::write(&file_path, template).expect("Failed to write day file");
    println!("Created {}", file_path);

    // 2. Create input file
    let input_path = format!("inputs/input_{}.txt", day_str);
    if !Path::new(&input_path).exists() {
        if let Some(parent) = Path::new(&input_path).parent() {
            fs::create_dir_all(parent).expect("Failed to create inputs directory");
        }
        fs::write(&input_path, "").expect("Failed to create input file");
        println!("Created {}", input_path);
    }

    // 3. Update main.rs
    let main_path = "src/main.rs";
    let main_content = fs::read_to_string(main_path).expect("Failed to read main.rs");
    let mut new_lines = Vec::new();
    let mut module_added = false;
    let mut dispatch_added = false;

    for line in main_content.lines() {
        new_lines.push(line.to_string());
        
        if line.contains("// MODULES") && !module_added {
            // Check if module is already declared (naive check)
            if !main_content.contains(&format!("mod {};", day_str)) {
                 new_lines.push(format!("mod {};", day_str));
                 println!("Added module declaration to main.rs");
            }
            module_added = true;
        }

        if line.contains("// DISPATCH") && !dispatch_added {
             // Check if dispatch is already there (naive check)
             // We just add it blindly if it's not strictly exact, relying on manual cleanup if dupes happen, 
             // but simpler: just insert it.
             // Better: scan if line "{} => {}::run()," exists.
             if !main_content.contains(&format!("{} => {}::run(),", day, day_str)) {
                 new_lines.push(format!("            {} => {}::run(),", day, day_str));
                 println!("Added dispatch arm to main.rs");
             }
             dispatch_added = true;
        }
    }
    
    // Just in case the simple loop above fails to place items correctly due to already existing items making logic weird,
    // we rewrite standardly. Actually, the above Logic inserts *after* the marker.
    // The marker is `// MODULES` -> insert `mod dayXX;`
    // The marker is `// DISPATCH` -> insert `X => dayX::run(),`
    
    // Let's refine the specific insertion to be strictly *after* the marker, and valid Rust.
    // `mod dayXX` should be at top level.
    // `dispatch` inside match.
    
    // Re-reading logic:
    // Pushes `// MODULES`
    // Then pushes `mod dayXX;`
    // This is correct order.
    
    fs::write(main_path, new_lines.join("\n")).expect("Failed to update main.rs");
}
