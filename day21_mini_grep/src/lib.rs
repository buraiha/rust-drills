pub fn grep_lines<'a>(pattern:&str, text:&'a str)->Vec<&'a str>{ text.lines().filter(|l| l.contains(pattern)).collect() }

#[cfg(test)]
mod tests{ use super::*; #[test] fn grep_works(){ let t="a\nhello\nworld"; assert_eq!(grep_lines("o",t), vec!["hello","world"]); } }
