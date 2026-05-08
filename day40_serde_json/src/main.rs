fn main(){ let u=day40_serde_json::User{name:"takashi".into(), age:20}; println!("{}", day40_serde_json::to_json(&u)); }
