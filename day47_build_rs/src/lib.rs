pub fn built_info() -> (&'static str, &'static str) {
    (env!("CARGO_PKG_VERSION"), env!("BUILD_TS"))
}
