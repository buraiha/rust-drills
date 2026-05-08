pub fn sum_of_squares(xs:&[i64])->i64{ xs.iter().map(|x| x*x).sum() }

pub fn moving_average(xs:&[f64], window:usize)->Vec<f64>{
    if window==0 || xs.len()<window { return vec![]; }
    (0..=xs.len()-window).map(|i| xs[i..i+window].iter().sum::<f64>()/window as f64).collect()
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn sos(){ assert_eq!(sum_of_squares(&[1,2,3]),14); }
    #[test] fn ma(){ let v=moving_average(&[1.0,2.0,3.0,4.0],2); assert_eq!(v, vec![1.5,2.5,3.5]); }
}
