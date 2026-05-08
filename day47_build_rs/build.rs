fn main() {
    let ts = chrono::Utc::now().to_rfc3339();
    println!("cargo:rustc-env=BUILD_TS={}", ts);
}
