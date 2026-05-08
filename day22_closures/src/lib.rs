pub fn filter_with<F>(xs:&[i32], pred:F)->Vec<i32>
where F:Fn(i32)->bool { xs.iter().copied().filter(|&x| pred(x)).collect() }

#[cfg(test)]
mod tests{ use super::*; #[test] fn closure_filter(){ let xs=[1,2,3,4]; let evens=filter_with(&xs, |x| x%2==0); assert_eq!(evens, vec![2,4]); } }
