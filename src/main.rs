use clap::Parser;
// use colored::*;
use std::time::Instant;

// MODULES
mod day05;
mod day01;
mod day02;
mod day03;
mod day04;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(required = false)]
    day: Option<u8>,

    /// Benchmark the solution
    #[arg(short, long)]
    bench: bool,

    /// Run all days
    #[arg(long)]
    all: bool,
}

fn main() {
    let args = Args::parse();
    
    let days_to_run = if args.all {
         (1..=25).collect()
    } else if let Some(day) = args.day {
        vec![day]
    } else {
        println!("Please provide a day to run or use --all");
        return;
    };

    let mut total_time = std::time::Duration::new(0, 0);

    for day in days_to_run {
        let start = Instant::now();
        match day {
            // DISPATCH
            5 => day05::run(),
            1 => day01::run(),
            2 => day02::run(),
            3 => day03::run(),
            4 => day04::run(),
            _ => {
                if args.all {
                     continue;
                } else {
                   eprintln!("Day {} not implemented or not added to main.rs", day);
                }
            }
        }
        let duration = start.elapsed();
        if args.bench {
            println!("Day {} took: {:?}", day, duration);
            total_time += duration;
        }
    }
    
    if args.bench && args.all {
        println!("\nTotal time: {:?}", total_time);
    }
}