pub fn shout_owned(s: String) -> String { s.to_uppercase() + "!!" }
pub fn shout_borrowed(s: &str) -> String { s.to_uppercase() + "!!" }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn owned_vs_borrowed(){
        let a=String::from("hello"); let b=shout_owned(a); assert_eq!(b,"HELLO!!");
        let c=String::from("world"); let d=shout_borrowed(&c); assert_eq!(d,"WORLD!!"); assert_eq!(c,"world");
    }
}
