fn main(){ let s="abc"; println!("safe={}", day49_unsafe_intro::first_byte(s)); unsafe{ println!("unsafe={}", day49_unsafe_intro::first_byte_unchecked(s)); } }
