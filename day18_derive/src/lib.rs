#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item{ pub id:u64, pub name:String }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn derives_help(){ let a=Item{id:1,name:"x".into()}; let b=a.clone(); assert_eq!(a,b); assert!(format!("{:?}", a).contains("Item")); }
}
