//! ドキュメントコメントの練習
//! 
//! `cargo test` で doctest も走ります。

/// 2つの整数を足します。
///
/// # Examples
///
/// ```
/// use day27_docs::add;
/// assert_eq!(add(1, 2), 3);
/// ```
pub fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests{ use super::*; #[test] fn add_works(){ assert_eq!(add(10,20),30); } }
