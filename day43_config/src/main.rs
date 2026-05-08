fn main(){ let args:Vec<String>=std::env::args().skip(1).collect(); let cfg=day43_config::load(&args); println!("{cfg:?}"); }
