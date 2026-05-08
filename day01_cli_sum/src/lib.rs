pub fn sum(nums: &[i64]) -> i64 {
    nums.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_works() { assert_eq!(sum(&[1,2,3]), 6); }
}
