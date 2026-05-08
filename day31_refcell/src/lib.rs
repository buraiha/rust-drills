use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct Counter{ pub n:usize }

pub fn shared_counter()->Rc<RefCell<Counter>>{ Rc::new(RefCell::new(Counter::default())) }

#[cfg(test)]
mod tests{ use super::*; #[test] fn increment(){ let c=shared_counter(); c.borrow_mut().n += 1; c.borrow_mut().n += 1; assert_eq!(c.borrow().n,2); } }
