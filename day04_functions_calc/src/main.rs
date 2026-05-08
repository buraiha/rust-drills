use std::process;
fn main(){
    let args: Vec<String> = std::env::args().collect();
    if args.len()!=4 { eprintln!("Usage: {} <a> <op> <b>", args.get(0).unwrap_or(&"day04".into())); process::exit(2); }
    let a: f64 = args[1].parse().expect("a");
    let op = day04_functions_calc::parse_op(&args[2]).expect("op");
    let b: f64 = args[3].parse().expect("b");
    match day04_functions_calc::calc(a, op, b) {
        Ok(v)=>println!("{v}"),
        Err(e)=>{ eprintln!("{e}"); process::exit(2); }
    }
}
