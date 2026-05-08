#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),* $(,)?) => {{
        let mut v = Vec::<String>::new();
        $( v.push($x.to_string()); )*
        v
    }};
}

#[cfg(test)]
mod tests{ #[test] fn macro_works(){ let v=day48_macros::vec_of_strings!["a","b"]; assert_eq!(v, vec!["a".to_string(),"b".to_string()]); } }
