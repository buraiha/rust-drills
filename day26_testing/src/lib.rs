pub fn clamp(x:i32, lo:i32, hi:i32)->i32{ if x<lo {lo} else if x>hi {hi} else {x} }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn table(){ let cases=vec![(0,1,10,1),(5,1,10,5),(99,1,10,10)]; for (x,lo,hi,exp) in cases { assert_eq!(clamp(x,lo,hi), exp); } }
}
