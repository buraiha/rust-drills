#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command { Add(String), Del(String), List }

pub fn parse_command(args: &[String]) -> Result<Command, String> {
    match args.get(0).map(|s| s.as_str()) {
        Some("add")=>Ok(Command::Add(args.get(1).cloned().ok_or("missing item")?)),
        Some("del")=>Ok(Command::Del(args.get(1).cloned().ok_or("missing item")?)),
        Some("list")=>Ok(Command::List),
        _=>Err("usage: add <x> | del <x> | list".into()),
    }
}

#[cfg(test)]
mod tests{ use super::*;
    #[test] fn parse_add(){ let v=vec!["add".into(),"x".into()]; assert_eq!(parse_command(&v).unwrap(), Command::Add("x".into())); }
}
