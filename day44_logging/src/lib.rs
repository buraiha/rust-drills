pub fn init(){ let _=env_logger::builder().is_test(true).try_init(); }
pub fn do_work(){ log::info!("hello from info"); log::debug!("hello from debug"); log::error!("hello from error"); }

#[cfg(test)]
mod tests{ use super::*; #[test] fn smoke(){ init(); do_work(); } }
