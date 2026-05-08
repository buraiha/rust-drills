pub fn first_byte(s:&str)->u8{ s.as_bytes().get(0).copied().unwrap_or(0) }

pub unsafe fn first_byte_unchecked(s:&str)->u8{ *s.as_bytes().get_unchecked(0) }

#[cfg(test)]
mod tests{ use super::*; #[test] fn safe_ok(){ assert_eq!(first_byte("abc"), b'a'); assert_eq!(first_byte(""),0); } }
