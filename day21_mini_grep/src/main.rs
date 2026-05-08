use std::process;
fn main(){
    let mut it=std::env::args().skip(1);
    let pat=it.next().unwrap_or_else(||{ eprintln!("Usage: day21_mini_grep <pattern> <file>"); process::exit(2); });
    let file=it.next().unwrap_or_else(||{ eprintln!("Usage: day21_mini_grep <pattern> <file>"); process::exit(2); });
    let s=std::fs::read_to_string(file).expect("read file");
    for line in day21_mini_grep::grep_lines(&pat, &s){ println!("{line}"); }
}
