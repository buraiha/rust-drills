use std::collections::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Stats { pub lines: usize, pub words: usize, pub chars: usize, pub top: Vec<(String, usize)> }

pub fn compute(s: &str, top_n: usize) -> Stats {
    let lines = s.lines().count();
    let words_vec: Vec<&str> = s.split_whitespace().collect();
    let words = words_vec.len();
    let chars = s.chars().count();
    let mut freq: HashMap<String, usize> = HashMap::new();
    for w in words_vec { *freq.entry(w.to_lowercase()).or_insert(0) += 1; }
    let mut top: Vec<(String, usize)> = freq.into_iter().collect();
    top.sort_by(|a,b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    top.truncate(top_n);
    Stats { lines, words, chars, top }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn stats_basic(){ let s="a a\nb"; let st=compute(s,3); assert_eq!(st.lines,2); assert_eq!(st.words,3); assert_eq!(st.chars,5); }
}
