use std::sync::{Arc,Mutex};
use std::thread;

pub fn parallel_inc(n_threads:usize, n_each:usize)->usize{
    let c=Arc::new(Mutex::new(0usize));
    let mut handles=vec![];
    for _ in 0..n_threads{
        let c2=c.clone();
        handles.push(thread::spawn(move ||{ for _ in 0..n_each { *c2.lock().unwrap() += 1; } }));
    }
    for h in handles { h.join().unwrap(); }
    *c.lock().unwrap()
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn counts(){ assert_eq!(parallel_inc(4,1000),4000); } }
