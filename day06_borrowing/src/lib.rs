pub fn mean(xs: &[i64]) -> Option<f64> {
    if xs.is_empty(){ return None; }
    let sum: i64 = xs.iter().sum();
    Some(sum as f64 / xs.len() as f64)
}

pub fn scale_in_place(xs: &mut [i64], k: i64){ for x in xs.iter_mut(){ *x *= k; } }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn mean_works(){ assert_eq!(mean(&[1,2,3]).unwrap(),2.0); assert!(mean(&[]).is_none()); }
    #[test] fn scale(){ let mut v=vec![1,2,3]; scale_in_place(&mut v,2); assert_eq!(v, vec![2,4,6]); }
}
