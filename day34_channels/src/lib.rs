use std::sync::mpsc;
use std::thread;

pub fn sum_via_channel(n:usize)->i64{
    let (tx,rx)=mpsc::channel::<i64>();
    let prod=thread::spawn(move ||{ for i in 1..=n as i64 { tx.send(i).unwrap(); } });
    let mut total=0i64;
    for v in rx { total += v; if v == n as i64 { break; } }
    prod.join().unwrap();
    total
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn sum10(){ assert_eq!(sum_via_channel(10),55); } }
