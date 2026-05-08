use std::process;
fn main(){
    let args: Vec<String> = std::env::args().skip(1).collect();
    let cmd = match day09_enums::parse_command(&args) {
        Ok(c)=>c,
        Err(e)=>{ eprintln!("{e}"); process::exit(2); }
    };
    println!("{cmd:?}");
}
