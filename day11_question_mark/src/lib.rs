use std::fs;
use std::path::Path;

pub fn sum_file(path: impl AsRef<Path>) -> Result<i64, Box<dyn std::error::Error>> {
    let s = fs::read_to_string(path)?;
    let mut total = 0i64;
    for (i, line) in s.lines().enumerate() {
        let n: i64 = line.trim().parse().map_err(|e| format!("line {}: parse error: {}", i+1, e))?;
        total += n;
    }
    Ok(total)
}

#[cfg(test)]
mod tests{ use super::*; use std::io::Write;
    #[test] fn sum_file_works(){
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        let mut f = std::fs::File::create(&file).unwrap();
        writeln!(f, "1\n2\n3").unwrap();
        assert_eq!(sum_file(&file).unwrap(), 6);
    }
}
