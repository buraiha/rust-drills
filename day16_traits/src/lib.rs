pub trait Describe{ fn describe(&self)->String; }

pub struct User{ pub name:String }
pub struct Server{ pub host:String, pub port:u16 }

impl Describe for User{ fn describe(&self)->String{ format!("User(name={})", self.name) } }
impl Describe for Server{ fn describe(&self)->String{ format!("Server({}:{})", self.host, self.port) } }

pub fn describe_all<T: Describe>(xs:&[T])->Vec<String>{ xs.iter().map(|x|x.describe()).collect() }

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn works(){ let u=User{name:"a".into()}; let s=Server{host:"localhost".into(), port:80}; assert_eq!(u.describe(),"User(name=a)"); assert_eq!(s.describe(),"Server(localhost:80)"); }
}
