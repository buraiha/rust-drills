fn main(){ let t=day29_box_tree::Tree::Node(Box::new(day29_box_tree::Tree::Leaf(10)), Box::new(day29_box_tree::Tree::Leaf(20))); println!("{}", day29_box_tree::sum(&t)); }
