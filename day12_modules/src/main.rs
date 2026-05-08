fn main(){
    let s="10 + 20";
    let (a,op,b) = day12_modules::parser::parse(s).unwrap();
    let v = day12_modules::calc::eval(a,op,b).unwrap();
    println!("{v}");
}
