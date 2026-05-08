#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User { pub name: String, pub age: u8 }

impl User {
    pub fn new(name: impl Into<String>, age: u8) -> Self { Self { name: name.into(), age } }
    pub fn is_adult(&self) -> bool { self.age >= 18 }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn adult(){ assert!(User::new("a",18).is_adult()); assert!(!User::new("b",17).is_adult()); }
}
