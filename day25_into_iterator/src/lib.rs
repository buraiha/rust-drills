pub fn sum_by_value(v:Vec<i32>)->i32{ v.into_iter().sum() }
pub fn sum_by_ref(v:&[i32])->i32{ v.iter().sum() }
pub fn inc_all(v:&mut [i32]){ for x in v.iter_mut(){ *x += 1; } }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn ownership_modes(){ let v=vec![1,2,3]; assert_eq!(sum_by_ref(&v),6); let s=sum_by_value(v); assert_eq!(s,6); }
    #[test] fn mut_iter(){ let mut v=vec![1,2]; inc_all(&mut v); assert_eq!(v, vec![2,3]); }
}
