#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op { Add, Sub, Mul, Div }

impl Op {
    pub fn from_str(s: &str) -> Option<Self> {
        match s { "+"=>Some(Self::Add), "-"=>Some(Self::Sub), "*"=>Some(Self::Mul), "/"=>Some(Self::Div), _=>None }
    }
}

pub fn eval(a: f64, op: Op, b: f64) -> Result<f64, String> {
    match op {
        Op::Add=>Ok(a+b),
        Op::Sub=>Ok(a-b),
        Op::Mul=>Ok(a*b),
        Op::Div=> if b==0.0 { Err("div0".into()) } else { Ok(a/b) },
    }
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn add(){ assert_eq!(eval(1.0,Op::Add,2.0).unwrap(), 3.0); } }
