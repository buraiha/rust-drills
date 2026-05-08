use std::rc::Rc;

#[derive(Debug)]
pub struct Node{ pub name:String, pub next: Option<Rc<Node>> }

pub fn make_chain()->Rc<Node>{
    let c=Rc::new(Node{name:"c".into(), next:None});
    let b=Rc::new(Node{name:"b".into(), next:Some(c.clone())});
    Rc::new(Node{name:"a".into(), next:Some(b.clone())})
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn rc_counts(){ let a=make_chain(); assert_eq!(Rc::strong_count(&a),1); let _a2=a.clone(); assert_eq!(Rc::strong_count(&a),2); } }
