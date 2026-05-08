use std::thread;

pub fn count_words_parallel(s:&str, n_threads:usize)->usize{
    if n_threads==0 { return 0; }
    let len=s.len();
    let chunk=(len + n_threads - 1)/n_threads;
    let mut handles=Vec::new();
    for i in 0..n_threads{
        let start=i*chunk;
        if start>=len { break; }
        let end=((i+1)*chunk).min(len);
        let part=s[start..end].to_string();
        handles.push(thread::spawn(move || part.split_whitespace().count()));
    }
    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn parallel_counts(){ let s="a b c d e"; let c=count_words_parallel(s,3); assert!(c>=5); } }
