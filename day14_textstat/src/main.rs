use std::process;
fn main(){
    let path=std::env::args().nth(1).unwrap_or_else(||{ eprintln!("Usage: day14_textstat <file>"); process::exit(2); });
    let s=std::fs::read_to_string(path).expect("read file");
    let st=day14_textstat::compute(&s,10);
    println!("lines: {}", st.lines);
    println!("words: {}", st.words);
    println!("chars: {}", st.chars);
    println!("top: {:?}", st.top);
}
