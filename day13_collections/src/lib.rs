use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in s.split_whitespace() { *m.entry(w.to_lowercase()).or_insert(0) += 1; }
    m
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn count(){ let m=word_count("a b a"); assert_eq!(m.get("a"),Some(&2)); assert_eq!(m.get("b"),Some(&1)); } }
