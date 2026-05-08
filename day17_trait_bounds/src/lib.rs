pub fn max_by_key<T,K,F>(xs:&[T], key:F)->Option<&T>
where F:Fn(&T)->K, K:Ord {
    let mut best: Option<&T> = None;
    for x in xs {
        best = match best { None => Some(x), Some(b) => if key(x) > key(b) { Some(x) } else { Some(b) } };
    }
    best
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn max_works(){ let xs=vec!["a","bbb","cc"]; let m=max_by_key(&xs, |s| s.len()).unwrap(); assert_eq!(*m, "bbb"); }
}
