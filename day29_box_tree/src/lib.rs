#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Tree{ Leaf(i32), Node(Box<Tree>, Box<Tree>) }

pub fn sum(t:&Tree)->i32{ match t { Tree::Leaf(x)=>*x, Tree::Node(a,b)=>sum(a)+sum(b) } }

#[cfg(test)]
mod tests{ use super::*; #[test] fn tree_sum(){ let t=Tree::Node(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2))); assert_eq!(sum(&t),3); } }
