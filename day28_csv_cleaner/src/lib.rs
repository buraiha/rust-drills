pub fn clean_line(line:&str)->Vec<String>{ line.split(',').map(|s| s.trim().to_string()).collect() }

#[cfg(test)]
mod tests{ use super::*; #[test] fn clean(){ assert_eq!(clean_line(" a, b ,c "), vec!["a","b","c"]); } }
