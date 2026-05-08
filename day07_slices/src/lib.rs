pub fn first_word(s: &str) -> &str {
    match s.find(char::is_whitespace) {
        Some(i)=>&s[..i],
        None=>s,
    }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn first_word_works(){ assert_eq!(first_word("hello world"),"hello"); assert_eq!(first_word("hello"),"hello"); assert_eq!(first_word("  leading"),""); }
}
