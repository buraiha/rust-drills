pub fn take_chars(s:&str, n:usize)->String{ s.chars().take(n).collect() }

#[cfg(test)]
mod tests{ use super::*; #[test] fn utf8_safe(){ assert_eq!(take_chars("あいうえお",3),"あいう"); } }
