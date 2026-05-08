use crate::calc::Op;

pub fn parse(s: &str) -> Result<(f64, Op, f64), String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len()!=3 { return Err("expected: <a> <op> <b>".into()); }
    let a: f64 = parts[0].parse().map_err(|_| "bad a")?;
    let op = Op::from_str(parts[1]).ok_or("bad op")?;
    let b: f64 = parts[2].parse().map_err(|_| "bad b")?;
    Ok((a, op, b))
}
