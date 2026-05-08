pub fn fizzbuzz(n: u32) -> Vec<String> {
    let mut out = Vec::with_capacity(n as usize);
    for i in 1..=n {
        let s = match (i%3, i%5) {
            (0,0)=>"FizzBuzz".to_string(),
            (0,_)=>"Fizz".to_string(),
            (_,0)=>"Buzz".to_string(),
            _=>i.to_string(),
        };
        out.push(s);
    }
    out
}

pub fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    let mut d = 2;
    while d*d <= n {
        if n % d == 0 { return false; }
        d += 1;
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn fb15(){ let v=fizzbuzz(15); assert_eq!(v[2],"Fizz"); assert_eq!(v[4],"Buzz"); assert_eq!(v[14],"FizzBuzz"); }
    #[test]
    fn primes(){ assert!(is_prime(2)); assert!(is_prime(13)); assert!(!is_prime(1)); assert!(!is_prime(12)); }
}
