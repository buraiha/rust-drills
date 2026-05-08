use serde::{Deserialize,Serialize};

#[derive(Debug,Serialize,Deserialize,PartialEq,Eq)]
pub struct User{ pub name:String, pub age:u8 }

pub fn to_json(u:&User)->String{ serde_json::to_string(u).unwrap() }
pub fn from_json(s:&str)->User{ serde_json::from_str(s).unwrap() }

#[cfg(test)]
mod tests{ use super::*; #[test] fn roundtrip(){ let u=User{name:"a".into(), age:1}; let s=to_json(&u); let u2=from_json(&s); assert_eq!(u,u2); } }
