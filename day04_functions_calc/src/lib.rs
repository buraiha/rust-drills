#[derive(Debug, PartialEq)]
pub enum Op { Add, Sub, Mul, Div }

pub fn parse_op(s: &str) -> Option<Op> {
    match s {
        "+"=>Some(Op::Add),
        "-"=>Some(Op::Sub),
        "*"=>Some(Op::Mul),
        "/"=>Some(Op::Div),
        _=>None,
    }
}

pub fn calc(a: f64, op: Op, b: f64) -> Result<f64, String> {
    match op {
        Op::Add => Ok(a+b),
        Op::Sub => Ok(a-b),
        Op::Mul => Ok(a*b),
        Op::Div => if b==0.0 { Err("division by zero".into()) } else { Ok(a/b) },
    }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn add(){ assert_eq!(calc(1.0,Op::Add,2.0).unwrap(),3.0); }
    #[test] fn div0(){ assert!(calc(1.0,Op::Div,0.0).is_err()); }
}
