use std::process;

fn print_usage(program: &str) {
    eprintln!("Usage: {program} <num> <num> ...");
    eprintln!("Example: {program} 10 -2 5");
}

fn main() {
    let mut args = std::env::args();
    let program = args.next().unwrap_or_else(|| "day01_cli_sum".to_string());
    let rest: Vec<String> = args.collect();
    if rest.is_empty() {
        print_usage(&program);
        process::exit(2);
    }
    let mut nums: Vec<i64> = Vec::with_capacity(rest.len());
    for s in rest {
        match s.parse::<i64>() {
            Ok(n) => nums.push(n),
            Err(_) => {
                eprintln!("Error: '{s}' is not a valid integer.");
                print_usage(&program);
                process::exit(2);
            }
        }
    }
    let total = day01_cli_sum::sum(&nums);
    println!("{total}");
}
