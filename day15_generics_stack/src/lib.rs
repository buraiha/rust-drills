#[derive(Debug, Default)]
pub struct Stack<T>{ items: Vec<T> }

impl<T> Stack<T>{
    pub fn new()->Self{ Self{ items: Vec::new() } }
    pub fn push(&mut self, v:T){ self.items.push(v); }
    pub fn pop(&mut self)->Option<T>{ self.items.pop() }
    pub fn peek(&self)->Option<&T>{ self.items.last() }
    pub fn len(&self)->usize{ self.items.len() }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn lifo(){ let mut s=Stack::new(); s.push(1); s.push(2); assert_eq!(s.peek(),Some(&2)); assert_eq!(s.pop(),Some(2)); assert_eq!(s.pop(),Some(1)); assert_eq!(s.pop(),None); }
}
