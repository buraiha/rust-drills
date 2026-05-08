#[cfg(feature = "mock")]
pub fn data_source()->&'static str{ "mock" }

#[cfg(not(feature = "mock"))]
pub fn data_source()->&'static str{ "real" }

#[cfg(test)]
mod tests{ use super::*; #[test] fn returns(){ assert!(!data_source().is_empty()); } }
