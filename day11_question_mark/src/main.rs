use std::process;
fn main(){
    let path = std::env::args().nth(1).unwrap_or_else(||{ eprintln!("Usage: day11_question_mark <file>"); process::exit(2); });
    match day11_question_mark::sum_file(path) {
        Ok(v)=>println!("{v}"),
        Err(e)=>{ eprintln!("{e}"); process::exit(2); }
    }
}
