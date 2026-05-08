#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Config{ pub port:u16 }

pub fn load(args:&[String])->Config{
    let mut port: Option<u16> = None;
    let mut it=args.iter();
    while let Some(a)=it.next(){
        if a=="--port" { if let Some(v)=it.next(){ port=v.parse().ok(); } }
    }
    if port.is_none(){ if let Ok(v)=std::env::var("PORT"){ port=v.parse().ok(); } }
    Config{ port: port.unwrap_or(3000) }
}

#[cfg(test)]
mod tests{ use super::*; #[test] fn arg_over_env(){ std::env::set_var("PORT","9999"); let args=vec!["--port".into(),"1234".into()]; assert_eq!(load(&args).port,1234); } }
