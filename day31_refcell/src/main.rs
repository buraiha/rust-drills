fn main(){ let c=day31_refcell::shared_counter(); c.borrow_mut().n += 1; println!("{}", c.borrow().n); }
