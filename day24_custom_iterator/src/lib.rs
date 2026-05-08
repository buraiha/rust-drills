#[derive(Debug, Clone)]
pub struct RangeStep{ cur:i64, end:i64, step:i64 }

impl RangeStep{ pub fn new(start:i64, end:i64, step:i64)->Self{ assert!(step!=0); Self{cur:start,end,step} } }

impl Iterator for RangeStep{
    type Item=i64;
    fn next(&mut self)->Option<Self::Item>{
        if (self.step>0 && self.cur>=self.end) || (self.step<0 && self.cur<=self.end){ return None; }
        let out=self.cur; self.cur += self.step; Some(out)
    }
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn step_iter(){ let v:Vec<i64>=RangeStep::new(0,5,2).collect(); assert_eq!(v, vec![0,2,4]); } }
