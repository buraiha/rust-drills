use std::fmt;

#[derive(Debug)]
pub enum AppError{ ParseInt(std::num::ParseIntError), Empty }

impl fmt::Display for AppError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>)->fmt::Result{
        match self{ AppError::ParseInt(e)=>write!(f,"parse error: {e}"), AppError::Empty=>write!(f,"empty input") }
    }
}

impl std::error::Error for AppError{}

impl From<std::num::ParseIntError> for AppError{ fn from(e:std::num::ParseIntError)->Self{ AppError::ParseInt(e) } }

pub fn parse_nonempty_i32(s:&str)->Result<i32,AppError>{ if s.is_empty(){ return Err(AppError::Empty); } Ok(s.parse::<i32>()?) }

#[cfg(test)]
mod tests{ use super::*; #[test] fn paths(){ assert!(parse_nonempty_i32("").is_err()); assert!(parse_nonempty_i32("x").is_err()); assert_eq!(parse_nonempty_i32("10").unwrap(),10); } }
