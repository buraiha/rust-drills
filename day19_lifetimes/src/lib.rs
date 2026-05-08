pub fn longest<'a>(a:&'a str, b:&'a str)->&'a str{ if a.len()>=b.len(){ a } else { b } }

#[cfg(test)]
mod tests{ use super::*; #[test] fn longest_works(){ assert_eq!(longest("abc","x"),"abc"); } }
