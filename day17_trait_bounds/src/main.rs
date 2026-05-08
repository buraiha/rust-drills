fn main(){ let xs=vec![10,5,20]; let m=day17_trait_bounds::max_by_key(&xs, |x| *x).unwrap(); println!("{m}"); }
