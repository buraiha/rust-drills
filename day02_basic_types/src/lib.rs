pub fn c_to_f(c: f64) -> f64 { c * 9.0 / 5.0 + 32.0 }
pub fn f_to_c(f: f64) -> f64 { (f - 32.0) * 5.0 / 9.0 }
pub fn bmi(height_m: f64, weight_kg: f64) -> f64 { weight_kg / (height_m * height_m) }
pub fn gb_to_bytes(gb: u64) -> u64 { gb * 1024 * 1024 * 1024 }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn temp_roundtrip(){ let c=25.0; let f=c_to_f(c); let c2=f_to_c(f); assert!((c-c2).abs()<1e-9); }
    #[test]
    fn bmi_works(){ let v=bmi(1.70,60.0); assert!((v-20.761).abs()<0.01); }
    #[test]
    fn gb_bytes(){ assert_eq!(gb_to_bytes(1), 1024*1024*1024); }
}
