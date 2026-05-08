pub fn find_index(xs: &[i32], target: i32) -> Option<usize> { xs.iter().position(|&x| x==target) }
pub fn parse_i32(s: &str) -> Result<i32, String> { s.parse::<i32>().map_err(|e| e.to_string()) }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn find(){ assert_eq!(find_index(&[1,2,3],2), Some(1)); assert_eq!(find_index(&[1,2,3],9), None); }
    #[test] fn parse(){ assert_eq!(parse_i32("10").unwrap(),10); assert!(parse_i32("x").is_err()); }
}
